use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    Row, SqlitePool,
};
use std::str::FromStr;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanRow {
    pub id: String,
    pub target_domain: String,
    pub scan_module: String,
    pub status: String,
    pub error_message: Option<String>,
    pub is_favorite: bool,
    pub duration_ms: i64,
    pub started_at: String,
    pub finished_at: Option<String>,
}

pub async fn init_db(app_handle: &AppHandle) -> Result<(), AppError> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    std::fs::create_dir_all(&app_dir).map_err(|e| AppError::System(e.to_string()))?;
    
    let db_path = app_dir.join("webq_telemetry.db");
    
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());
    
    if !db_path.exists() {
        std::fs::File::create(&db_path).map_err(|e| AppError::System(e.to_string()))?;
    }

    let connection_options = SqliteConnectOptions::from_str(&db_url)
        .map_err(|e| AppError::System(e.to_string()))?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .map_err(|e| AppError::System(e.to_string()))?;

    // Create Base `scans` table
    let scans_schema = r#"
        CREATE TABLE IF NOT EXISTS scans (
            id TEXT PRIMARY KEY,
            target_domain TEXT NOT NULL,
            scan_module TEXT NOT NULL,
            status TEXT NOT NULL,
            error_message TEXT,
            config_options TEXT,
            is_favorite INTEGER NOT NULL DEFAULT 0,
            duration_ms INTEGER NOT NULL DEFAULT 0,
            started_at TEXT NOT NULL,
            finished_at TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_scans_domain ON scans(target_domain);
        CREATE INDEX IF NOT EXISTS idx_scans_module ON scans(scan_module);
    "#;

    sqlx::query(scans_schema)
        .execute(&pool)
        .await
        .map_err(|e| AppError::System(e.to_string()))?;

    // Create Secondary `scan_results` table
    let scan_results_schema = r#"
        CREATE TABLE IF NOT EXISTS scan_results (
            id TEXT PRIMARY KEY,
            scan_id TEXT NOT NULL,
            security_score INTEGER,
            critical_vulns INTEGER NOT NULL DEFAULT 0,
            high_vulns INTEGER NOT NULL DEFAULT 0,
            medium_vulns INTEGER NOT NULL DEFAULT 0,
            low_vulns INTEGER NOT NULL DEFAULT 0,
            raw_json_blob TEXT NOT NULL,
            FOREIGN KEY(scan_id) REFERENCES scans(id) ON DELETE CASCADE
        );
    "#;

    sqlx::query(scan_results_schema)
        .execute(&pool)
        .await
        .map_err(|e| AppError::System(e.to_string()))?;

    // Ensure foreign key tracking is activated
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .map_err(|e| AppError::System(e.to_string()))?;

    app_handle.manage(pool);

    Ok(())
}

#[tauri::command]
pub async fn get_scans_paginated(
    pool: State<'_, SqlitePool>,
    limit: i64,
    offset: i64,
) -> Result<Vec<ScanRow>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT id, target_domain, scan_module, status, error_message, is_favorite, duration_ms, started_at, finished_at
        FROM scans
        ORDER BY started_at DESC
        LIMIT ? OFFSET ?
        "#,
        limit, offset
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| AppError::System(e.to_string()))?;

    Ok(rows.into_iter().map(|r| ScanRow {
        id: r.id.unwrap_or_default(),
        target_domain: r.target_domain,
        scan_module: r.scan_module,
        status: r.status,
        error_message: r.error_message,
        is_favorite: r.is_favorite != 0,
        duration_ms: r.duration_ms,
        started_at: r.started_at,
        finished_at: r.finished_at,
    }).collect())
}

#[tauri::command]
pub async fn delete_scan(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), AppError> {
    sqlx::query!("DELETE FROM scans WHERE id = ?", id)
        .execute(&*pool)
        .await
        .map_err(|e| AppError::System(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn get_scan_blob_details(
    pool: State<'_, SqlitePool>,
    scan_id: String,
) -> Result<Value, AppError> {
    let row = sqlx::query!(
        r#"
        SELECT raw_json_blob
        FROM scan_results
        WHERE scan_id = ?
        LIMIT 1
        "#,
        scan_id
    )
    .fetch_one(&*pool)
    .await
    .map_err(|e| AppError::System(e.to_string()))?;

    let json_val: Value = serde_json::from_str(&row.raw_json_blob)
        .map_err(|e| AppError::System(e.to_string()))?;

    Ok(json_val)
}

pub async fn log_scan_to_db(
    pool: &SqlitePool,
    target_domain: &str,
    scan_module: &str,
    status: &str,
    error_message: Option<&str>,
    duration_ms: i64,
    critical_vulns: i32,
    high_vulns: i32,
    raw_json_blob: &serde_json::Value
) -> Result<String, AppError> {
    let scan_id = Uuid::new_v4().to_string();
    let result_id = Uuid::new_v4().to_string();
    // Using simple format since `chrono` is available
    let now = chrono::Utc::now().to_rfc3339();
    
    let blob_str = serde_json::to_string(raw_json_blob)
        .map_err(|e| AppError::System(e.to_string()))?;

    sqlx::query!(
        r#"INSERT INTO scans (id, target_domain, scan_module, status, error_message, duration_ms, started_at, finished_at) 
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        scan_id, target_domain, scan_module, status, error_message, duration_ms, now, now
    ).execute(pool).await.map_err(|e| AppError::System(e.to_string()))?;

    sqlx::query!(
        r#"INSERT INTO scan_results (id, scan_id, critical_vulns, high_vulns, raw_json_blob)
           VALUES (?, ?, ?, ?, ?)"#,
        result_id, scan_id, critical_vulns, high_vulns, blob_str
    ).execute(pool).await.map_err(|e| AppError::System(e.to_string()))?;

    Ok(scan_id)
}

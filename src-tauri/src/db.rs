use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    QueryBuilder, Row, Sqlite, SqlitePool,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalStatistics {
    pub total_scans: i64,
    pub completed_scans: i64,
    pub failed_scans: i64,
    pub favorite_scans: i64,
    pub average_security_score: Option<f64>,
    pub critical_vulns: i64,
    pub high_vulns: i64,
    pub medium_vulns: i64,
    pub low_vulns: i64,
    pub top_domains: Vec<UsageCount>,
    pub module_usage: Vec<UsageCount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageCount {
    pub name: String,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoricalScanHydration {
    pub started_at: String,
    pub duration_ms: i64,
    pub raw_json_blob: Value,
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
    filter_domain: Option<String>,
    filter_module: Option<String>,
    filter_status: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    sort_by: Option<String>,
    sort_dir: Option<String>,
) -> Result<Vec<ScanRow>, AppError> {
    let mut builder: QueryBuilder<'_, Sqlite> = QueryBuilder::new(
        "SELECT id, target_domain, scan_module, status, error_message, is_favorite, duration_ms, started_at, finished_at FROM scans WHERE 1=1"
    );

    if let Some(domain) = filter_domain.filter(|value| !value.trim().is_empty()) {
        builder.push(" AND target_domain LIKE ");
        builder.push_bind(format!("%{}%", domain.trim()));
    }

    if let Some(module) = filter_module.filter(|value| !value.trim().is_empty()) {
        builder.push(" AND scan_module = ");
        builder.push_bind(module);
    }

    if let Some(status) = filter_status.filter(|value| !value.trim().is_empty()) {
        builder.push(" AND status = ");
        builder.push_bind(status);
    }

    if let Some(from) = date_from.filter(|value| !value.trim().is_empty()) {
        builder.push(" AND started_at >= ");
        builder.push_bind(from);
    }

    if let Some(to) = date_to.filter(|value| !value.trim().is_empty()) {
        builder.push(" AND started_at <= ");
        builder.push_bind(to);
    }

    let sort_column = match sort_by.as_deref() {
        Some("target_domain") => "target_domain",
        Some("scan_module") => "scan_module",
        Some("status") => "status",
        Some("duration_ms") => "duration_ms",
        Some("is_favorite") => "is_favorite",
        _ => "started_at",
    };
    let sort_direction = if matches!(sort_dir.as_deref(), Some("asc") | Some("ASC")) {
        "ASC"
    } else {
        "DESC"
    };

    builder.push(" ORDER BY ");
    builder.push(sort_column);
    builder.push(" ");
    builder.push(sort_direction);
    builder.push(" LIMIT ");
    builder.push_bind(limit.clamp(1, 100));
    builder.push(" OFFSET ");
    builder.push_bind(offset.max(0));

    let rows = builder
        .build()
        .fetch_all(&*pool)
        .await
        .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|r| ScanRow {
            id: r.get("id"),
            target_domain: r.get("target_domain"),
            scan_module: r.get("scan_module"),
            status: r.get("status"),
            error_message: r.get("error_message"),
            is_favorite: r.get::<i64, _>("is_favorite") != 0,
            duration_ms: r.get("duration_ms"),
            started_at: r.get("started_at"),
            finished_at: r.get("finished_at"),
        })
        .collect())
}

#[tauri::command]
pub async fn get_global_statistics(
    pool: State<'_, SqlitePool>,
) -> Result<GlobalStatistics, AppError> {
    let status_row = sqlx::query(
        r#"
        SELECT
            COUNT(*) AS total_scans,
            SUM(CASE WHEN status = 'Completed' THEN 1 ELSE 0 END) AS completed_scans,
            SUM(CASE WHEN status = 'Failed' THEN 1 ELSE 0 END) AS failed_scans,
            SUM(CASE WHEN is_favorite = 1 THEN 1 ELSE 0 END) AS favorite_scans
        FROM scans
        "#,
    )
    .fetch_one(&*pool)
    .await
    .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    let vuln_row = sqlx::query(
        r#"
        SELECT
            AVG(security_score) AS average_security_score,
            SUM(critical_vulns) AS critical_vulns,
            SUM(high_vulns) AS high_vulns,
            SUM(medium_vulns) AS medium_vulns,
            SUM(low_vulns) AS low_vulns
        FROM scan_results
        "#,
    )
    .fetch_one(&*pool)
    .await
    .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    let top_domains = usage_counts(&pool, "target_domain").await?;
    let module_usage = usage_counts(&pool, "scan_module").await?;

    Ok(GlobalStatistics {
        total_scans: status_row.get::<i64, _>("total_scans"),
        completed_scans: status_row
            .get::<Option<i64>, _>("completed_scans")
            .unwrap_or(0),
        failed_scans: status_row
            .get::<Option<i64>, _>("failed_scans")
            .unwrap_or(0),
        favorite_scans: status_row
            .get::<Option<i64>, _>("favorite_scans")
            .unwrap_or(0),
        average_security_score: vuln_row.get::<Option<f64>, _>("average_security_score"),
        critical_vulns: vuln_row
            .get::<Option<i64>, _>("critical_vulns")
            .unwrap_or(0),
        high_vulns: vuln_row.get::<Option<i64>, _>("high_vulns").unwrap_or(0),
        medium_vulns: vuln_row.get::<Option<i64>, _>("medium_vulns").unwrap_or(0),
        low_vulns: vuln_row.get::<Option<i64>, _>("low_vulns").unwrap_or(0),
        top_domains,
        module_usage,
    })
}

async fn usage_counts(pool: &SqlitePool, column: &str) -> Result<Vec<UsageCount>, AppError> {
    let query = format!(
        "SELECT {column} AS name, COUNT(*) AS count FROM scans GROUP BY {column} ORDER BY count DESC, name ASC LIMIT 8"
    );
    let rows = sqlx::query(&query)
        .fetch_all(pool)
        .await
        .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|row| UsageCount {
            name: row.get("name"),
            count: row.get("count"),
        })
        .collect())
}

#[tauri::command]
pub async fn get_latest_domain_intel(
    pool: State<'_, SqlitePool>,
    domain: String,
    scan_module: String,
) -> Result<Option<HistoricalScanHydration>, AppError> {
    let row_opt = sqlx::query(
        r#"
        SELECT s.started_at, s.duration_ms, sr.raw_json_blob
        FROM scans s
        JOIN scan_results sr ON sr.scan_id = s.id
        WHERE s.target_domain = ? AND s.scan_module = ? AND s.status = 'Completed'
        ORDER BY s.started_at DESC
        LIMIT 1
        "#,
    )
    .bind(domain)
    .bind(scan_module)
    .fetch_optional(&*pool)
    .await
    .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    if let Some(row) = row_opt {
        let blob_str: String = row.get("raw_json_blob");
        if let Ok(json_val) = serde_json::from_str(&blob_str) {
            return Ok(Some(HistoricalScanHydration {
                started_at: row.get("started_at"),
                duration_ms: row.get("duration_ms"),
                raw_json_blob: json_val,
            }));
        }
    }

    Ok(None)
}

#[tauri::command]
pub async fn delete_scan(pool: State<'_, SqlitePool>, id: String) -> Result<(), AppError> {
    sqlx::query("DELETE FROM scans WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_favorite(pool: State<'_, SqlitePool>, id: String) -> Result<bool, AppError> {
    let row = sqlx::query("SELECT is_favorite FROM scans WHERE id = ?")
        .bind(&id)
        .fetch_one(&*pool)
        .await
        .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    let next = if row.get::<i64, _>("is_favorite") == 0 {
        1
    } else {
        0
    };
    sqlx::query("UPDATE scans SET is_favorite = ? WHERE id = ?")
        .bind(next)
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    Ok(next == 1)
}

#[tauri::command]
pub async fn bulk_delete_scans(
    pool: State<'_, SqlitePool>,
    ids: Vec<String>,
) -> Result<u64, AppError> {
    if ids.is_empty() {
        return Ok(0);
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;
    let mut deleted = 0;

    for id in ids {
        let result = sqlx::query("DELETE FROM scans WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;
        deleted += result.rows_affected();
    }

    tx.commit()
        .await
        .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    Ok(deleted)
}

#[tauri::command]
pub async fn get_scan_blob_details(
    pool: State<'_, SqlitePool>,
    scan_id: String,
) -> Result<Value, AppError> {
    let row = sqlx::query(
        r#"
        SELECT raw_json_blob
        FROM scan_results
        WHERE scan_id = ?
        LIMIT 1
        "#,
    )
    .bind(scan_id)
    .fetch_one(&*pool)
    .await
    .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    let blob_str: String = row.get("raw_json_blob");
    let json_val: Value =
        serde_json::from_str(&blob_str).map_err(|e| AppError::System(e.to_string()))?;

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
    raw_json_blob: &serde_json::Value,
) -> Result<String, AppError> {
    let scan_id = Uuid::new_v4().to_string();
    let result_id = Uuid::new_v4().to_string();
    // Using simple format since `chrono` is available
    let now = chrono::Utc::now().to_rfc3339();

    let blob_str =
        serde_json::to_string(raw_json_blob).map_err(|e| AppError::System(e.to_string()))?;
    let security_score = extract_security_score(raw_json_blob);
    let vuln_counts = extract_vulnerability_counts(raw_json_blob);
    let critical_vulns = if critical_vulns > 0 {
        critical_vulns
    } else {
        vuln_counts.0
    };
    let high_vulns = if high_vulns > 0 {
        high_vulns
    } else {
        vuln_counts.1
    };

    sqlx::query(
        r#"INSERT INTO scans (id, target_domain, scan_module, status, error_message, duration_ms, started_at, finished_at) 
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&scan_id)
    .bind(target_domain)
    .bind(scan_module)
    .bind(status)
    .bind(error_message)
    .bind(duration_ms)
    .bind(&now)
    .bind(&now)
    .execute(pool).await.map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    sqlx::query(
        r#"INSERT INTO scan_results (id, scan_id, security_score, critical_vulns, high_vulns, medium_vulns, low_vulns, raw_json_blob)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&result_id)
    .bind(&scan_id)
    .bind(security_score)
    .bind(critical_vulns)
    .bind(high_vulns)
    .bind(vuln_counts.2)
    .bind(vuln_counts.3)
    .bind(blob_str)
    .execute(pool).await.map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    Ok(scan_id)
}

fn extract_security_score(value: &Value) -> Option<i64> {
    value
        .pointer("/security_score/overall_score")
        .and_then(Value::as_i64)
        .or_else(|| value.pointer("/security_score").and_then(Value::as_i64))
        .or_else(|| value.pointer("/score").and_then(Value::as_i64))
        .or_else(|| {
            value
                .pointer("/summary/security_score")
                .and_then(Value::as_i64)
        })
}

fn extract_vulnerability_counts(value: &Value) -> (i32, i32, i32, i32) {
    let mut counts = (0, 0, 0, 0);
    count_vulnerabilities(value, &mut counts);
    counts
}

fn count_vulnerabilities(value: &Value, counts: &mut (i32, i32, i32, i32)) {
    match value {
        Value::Array(items) => {
            for item in items {
                count_vulnerabilities(item, counts);
            }
        }
        Value::Object(map) => {
            if let Some(severity) = map
                .get("severity")
                .or_else(|| map.get("risk_level"))
                .or_else(|| map.get("confidence"))
                .and_then(Value::as_str)
            {
                match severity.to_ascii_lowercase().as_str() {
                    "critical" => counts.0 += 1,
                    "high" => counts.1 += 1,
                    "medium" | "moderate" => counts.2 += 1,
                    "low" => counts.3 += 1,
                    _ => {}
                }
            }

            for child in map.values() {
                count_vulnerabilities(child, counts);
            }
        }
        _ => {}
    }
}

#[derive(Serialize)]
pub struct DbStats {
    pub size_string: String,
    pub total_scans: i64,
}

#[tauri::command]
pub async fn get_db_stats(
    pool: State<'_, SqlitePool>,
    app_handle: AppHandle,
) -> Result<DbStats, AppError> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM scans")
        .fetch_one(&*pool)
        .await
        .unwrap_or((0,));

    let app_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));

    let db_path = app_dir.join("webq_telemetry.db");
    let wal_path = app_dir.join("webq_telemetry.db-wal");
    let shm_path = app_dir.join("webq_telemetry.db-shm");

    let mut total_bytes = 0;

    if db_path.exists() {
        total_bytes += std::fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);
    }
    if wal_path.exists() {
        total_bytes += std::fs::metadata(&wal_path).map(|m| m.len()).unwrap_or(0);
    }
    if shm_path.exists() {
        total_bytes += std::fs::metadata(&shm_path).map(|m| m.len()).unwrap_or(0);
    }

    let size_mb = total_bytes as f64 / 1_048_576.0;

    Ok(DbStats {
        size_string: format!("{:.2} MB", size_mb),
        total_scans: row.0,
    })
}

#[tauri::command]
pub async fn nuke_history(pool: State<'_, SqlitePool>) -> Result<(), AppError> {
    sqlx::query("DELETE FROM scans")
        .execute(&*pool)
        .await
        .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    // Optional: Vacuum to reclaim disk space immediately
    let _ = sqlx::query("VACUUM;").execute(&*pool).await;

    Ok(())
}

#[tauri::command]
pub async fn get_unique_scanned_domains(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<String>, AppError> {
    let rows = sqlx::query(
        "SELECT target_domain FROM scans GROUP BY target_domain ORDER BY MAX(started_at) DESC LIMIT 50"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e: sqlx::Error| AppError::System(e.to_string()))?;

    Ok(rows.into_iter().map(|r| r.get("target_domain")).collect())
}

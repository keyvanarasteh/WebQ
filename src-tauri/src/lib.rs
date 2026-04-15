pub mod error;
pub mod system_health;
pub mod db;

use crate::error::AppError;
use tauri::Emitter;
use web_analyzer::domain_info::{get_domain_info, DomainInfoResult};
use web_analyzer::domain_dns::{get_dns_records, DomainDnsResult};
use web_analyzer::seo_analysis::{analyze_advanced_seo, SeoAnalysisResult};
use web_analyzer::web_technologies::{detect_web_technologies, WebTechResult};
use web_analyzer::domain_validator::{validate_domains_bulk, BulkValidationResult};
use web_analyzer::subdomain_discovery::{discover_subdomains, SubdomainDiscoveryResult};
use web_analyzer::contact_spy::{crawl_contacts, ContactSpyResult};
use web_analyzer::advanced_content_scanner::ScannerResult;
use web_analyzer::security_analysis::SecurityAnalysisResult;
use web_analyzer::subdomain_takeover::TakeoverResult;
use web_analyzer::cloudflare_bypass::CloudflareBypassResult;
use web_analyzer::nmap_zero_day::NmapScanResult;
use web_analyzer::api_security_scanner::ApiScanResult;
use web_analyzer::geo_analysis::GeoAnalysisResult;

#[derive(serde::Serialize)]
pub struct DependencyStatus {
    pub nmap: bool,
    pub dig: bool,
    pub openssl: bool,
}

#[tauri::command]
async fn check_dependencies() -> Result<DependencyStatus, AppError> {
    let nmap = std::process::Command::new("nmap")
        .arg("-V")
        .output()
        .is_ok();
    let dig = std::process::Command::new("dig")
        .arg("-v")
        .output()
        .is_ok();
    let openssl = std::process::Command::new("openssl")
        .arg("version")
        .output()
        .is_ok();
        
    Ok(DependencyStatus {
        nmap,
        dig,
        openssl,
    })
}

#[tauri::command]
fn get_system_status() -> String {
    "WebQ Target Secured".into()
}

macro_rules! log_and_execute_scan {
    ($pool:expr, $domain:expr, $module:expr, $future:expr) => {{
        let start_time = std::time::Instant::now();
        let result = $future.await;
        let duration = start_time.elapsed().as_millis() as i64;
        
        match result {
            Ok(data) => {
                let json = serde_json::to_value(&data).unwrap_or_default();
                let _ = db::log_scan_to_db(&*$pool, &$domain, $module, "Completed", None, duration, 0, 0, &json).await;
                Ok(data)
            },
            Err(e) => {
                let _ = db::log_scan_to_db(&*$pool, &$domain, $module, "Failed", Some(&e.to_string()), duration, 0, 0, &serde_json::json!({})).await;
                Err(AppError::ModuleFailed(e.to_string()))
            }
        }
    }};
}

#[tauri::command]
async fn scan_domain_info(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>, app_handle: tauri::AppHandle) -> Result<DomainInfoResult, AppError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_handle.emit("scan-progress", progress);
        }
    });
    log_and_execute_scan!(pool, domain, "DomainInfo", get_domain_info(&domain, Some(tx)))
}

#[tauri::command]
async fn scan_domain_dns(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>, app_handle: tauri::AppHandle) -> Result<DomainDnsResult, AppError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_handle.emit("scan-progress", progress);
        }
    });
    log_and_execute_scan!(pool, domain, "DomainDns", get_dns_records(&domain, Some(tx)))
}

#[tauri::command]
async fn scan_seo_analysis(url: String, pool: tauri::State<'_, sqlx::SqlitePool>, app_handle: tauri::AppHandle) -> Result<SeoAnalysisResult, AppError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_handle.emit("scan-progress", progress);
        }
    });
    log_and_execute_scan!(pool, url, "SeoAnalysis", analyze_advanced_seo(&url, Some(tx)))
}

#[tauri::command]
async fn scan_web_technologies(url: String, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<WebTechResult, AppError> {
    log_and_execute_scan!(pool, url, "WebTech", detect_web_technologies(&url))
}

#[tauri::command]
async fn validate_bulk_domains(domains: Vec<String>) -> Result<BulkValidationResult, AppError> {
    Ok(validate_domains_bulk(&domains, 10).await)
}

#[tauri::command]
async fn scan_subdomains(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<SubdomainDiscoveryResult, AppError> {
    log_and_execute_scan!(pool, domain, "SubdomainDiscovery", discover_subdomains(&domain))
}

#[tauri::command]
async fn scan_contacts(domain: String, max_pages: usize, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<ContactSpyResult, AppError> {
    log_and_execute_scan!(pool, domain, "ContactSpy", crawl_contacts(&domain, max_pages))
}

#[tauri::command]
async fn scan_advanced_content(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<ScannerResult, AppError> {
    log_and_execute_scan!(pool, domain, "AdvancedContent", web_analyzer::advanced_content_scanner::scan_content(&domain))
}

#[tauri::command]
async fn scan_security_posture(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<SecurityAnalysisResult, AppError> {
    log_and_execute_scan!(pool, domain, "SecurityAnalysis", web_analyzer::security_analysis::analyze_security(&domain))
}

#[tauri::command]
async fn scan_subdomain_takeover(domain: String, subdomains: Vec<String>, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<TakeoverResult, AppError> {
    log_and_execute_scan!(pool, domain, "Takeover", web_analyzer::subdomain_takeover::check_subdomain_takeover(&domain, &subdomains))
}

#[tauri::command]
async fn scan_cloudflare_bypass(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<CloudflareBypassResult, AppError> {
    log_and_execute_scan!(pool, domain, "CloudflareBypass", web_analyzer::cloudflare_bypass::find_real_ip(&domain))
}

#[tauri::command]
async fn scan_nmap_zero_day(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<NmapScanResult, AppError> {
    log_and_execute_scan!(pool, domain, "NmapZeroDay", web_analyzer::nmap_zero_day::run_nmap_scan(&domain))
}

#[tauri::command]
async fn scan_api_security(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<ApiScanResult, AppError> {
    log_and_execute_scan!(pool, domain, "ApiSecurity", web_analyzer::api_security_scanner::scan_api_endpoints(&domain))
}

#[tauri::command]
async fn scan_geo_analysis(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<GeoAnalysisResult, AppError> {
    log_and_execute_scan!(pool, domain, "GeoAnalysis", web_analyzer::geo_analysis::analyze_geo(&domain))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                db::init_db(app.handle()).await.expect("Failed to initialize SQLite database");
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            db::get_scans_paginated,
            db::delete_scan,
            db::get_scan_blob_details,
            check_dependencies,
            system_health::check_dependency,
            get_system_status,
            scan_domain_info,
            scan_domain_dns,
            scan_seo_analysis,
            scan_web_technologies,
            validate_bulk_domains,
            scan_subdomains,
            scan_contacts,
            scan_advanced_content,
            scan_security_posture,
            scan_subdomain_takeover,
            scan_cloudflare_bypass,
            scan_nmap_zero_day,
            scan_api_security,
            scan_geo_analysis
        ])
        .run(tauri::generate_context!())
        .expect("error while running WebQ application");
}

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
async fn scan_security_posture(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>, app_handle: tauri::AppHandle) -> Result<SecurityAnalysisResult, AppError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_handle.emit("scan-progress", progress);
        }
    });
    log_and_execute_scan!(pool, domain, "SecurityAnalysis", web_analyzer::security_analysis::analyze_security(&domain, Some(tx)))
}

#[tauri::command]
async fn scan_subdomain_takeover(domain: String, subdomains: Vec<String>, pool: tauri::State<'_, sqlx::SqlitePool>, app_handle: tauri::AppHandle) -> Result<TakeoverResult, AppError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_handle.emit("scan-progress", progress);
        }
    });
    log_and_execute_scan!(pool, domain, "Takeover", web_analyzer::subdomain_takeover::check_subdomain_takeover(&domain, &subdomains, Some(tx)))
}

#[tauri::command]
async fn scan_cloudflare_bypass(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>, app_handle: tauri::AppHandle) -> Result<CloudflareBypassResult, AppError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_handle.emit("scan-progress", progress);
        }
    });
    log_and_execute_scan!(pool, domain, "CloudflareBypass", web_analyzer::cloudflare_bypass::find_real_ip(&domain, Some(tx)))
}

#[tauri::command]
async fn scan_nmap_zero_day(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>, app_handle: tauri::AppHandle) -> Result<NmapScanResult, AppError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_handle.emit("scan-progress", progress);
        }
    });
    log_and_execute_scan!(pool, domain, "NmapZeroDay", web_analyzer::nmap_zero_day::run_nmap_scan(&domain, Some(tx)))
}

#[tauri::command]
async fn scan_api_security(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>, app_handle: tauri::AppHandle) -> Result<ApiScanResult, AppError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_handle.emit("scan-progress", progress);
        }
    });
    log_and_execute_scan!(pool, domain, "ApiSecurity", web_analyzer::api_security_scanner::scan_api_endpoints(&domain, Some(tx)))
}

#[tauri::command]
async fn scan_geo_analysis(domain: String, pool: tauri::State<'_, sqlx::SqlitePool>, app_handle: tauri::AppHandle) -> Result<GeoAnalysisResult, AppError> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            let _ = app_handle.emit("scan-progress", progress);
        }
    });
    log_and_execute_scan!(pool, domain, "GeoAnalysis", web_analyzer::geo_analysis::analyze_geo(&domain, Some(tx)))
}

        // -- Decoupled Domain Info Endpoints --
#[derive(serde::Serialize)]
pub struct IpResolutionInfo {
    ipv4: Option<String>,
    ipv6: Vec<String>,
    all_ipv4: Vec<String>,
    reverse_dns: Option<String>,
}

#[tauri::command]
async fn scan_ip_resolution(domain: String) -> Result<IpResolutionInfo, AppError> {
    let mut ipv4 = None;
    let mut all_ipv4 = vec![];
    let mut ipv6 = vec![];
    
    // We clean the domain exactly like web-analyzer does inside get_domain_info
    let d = domain.trim_start_matches("https://").trim_start_matches("http://").replace("www.", "");
    let clean = d.split('/').next().unwrap_or(&d).split(':').next().unwrap_or(&d).to_string();

    if let Ok(addrs) = tokio::net::lookup_host(format!("{}:80", clean)).await {
        for addr in addrs {
            match addr.ip() {
                std::net::IpAddr::V4(ip) => all_ipv4.push(ip.to_string()),
                std::net::IpAddr::V6(ip) => ipv6.push(ip.to_string()),
            }
        }
    }
    if !all_ipv4.is_empty() {
        ipv4 = Some(all_ipv4[0].clone());
    }
    
    let reverse_dns = if let Some(ref ip) = ipv4 {
        web_analyzer::domain_info::reverse_dns_lookup(ip).await
    } else {
        None
    };
    
    Ok(IpResolutionInfo { ipv4, ipv6, all_ipv4, reverse_dns })
}

#[tauri::command]
async fn scan_whois(domain: String) -> Result<web_analyzer::domain_info::WhoisInfo, AppError> {
    let d = domain.trim_start_matches("https://").trim_start_matches("http://").replace("www.", "");
    let clean = d.split('/').next().unwrap_or(&d).split(':').next().unwrap_or(&d).to_string();
    Ok(web_analyzer::domain_info::query_whois(&clean).await)
}

#[tauri::command]
async fn scan_ssl(domain: String) -> Result<web_analyzer::domain_info::SslInfo, AppError> {
    let d = domain.trim_start_matches("https://").trim_start_matches("http://").replace("www.", "");
    let clean = d.split('/').next().unwrap_or(&d).split(':').next().unwrap_or(&d).to_string();
    Ok(web_analyzer::domain_info::check_ssl(&clean).await)
}

#[tauri::command]
async fn scan_ports(ip: String) -> Result<Vec<String>, AppError> {
    Ok(web_analyzer::domain_info::scan_ports(Some(&ip)).await)
}

#[derive(serde::Serialize)]
pub struct HttpStatusInfo {
    status: Option<String>,
    server: Option<String>,
    response_time_ms: Option<f64>
}

#[tauri::command]
async fn scan_http_status(domain: String) -> Result<HttpStatusInfo, AppError> {
    let d = domain.trim_start_matches("https://").trim_start_matches("http://").replace("www.", "");
    let clean = d.split('/').next().unwrap_or(&d).split(':').next().unwrap_or(&d).to_string();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::limited(3))
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| AppError::ModuleFailed(e.to_string()))?;
        
    let res = web_analyzer::domain_info::check_http_status(&client, &clean).await;
    Ok(HttpStatusInfo {
        status: res.0,
        server: res.1,
        response_time_ms: res.2
    })
}

#[tauri::command]
async fn scan_security_headers(domain: String) -> Result<web_analyzer::domain_info::SecurityInfo, AppError> {
    let d = domain.trim_start_matches("https://").trim_start_matches("http://").replace("www.", "");
    let clean = d.split('/').next().unwrap_or(&d).split(':').next().unwrap_or(&d).to_string();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::limited(3))
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| AppError::ModuleFailed(e.to_string()))?;
        
    Ok(web_analyzer::domain_info::check_security(&client, &clean).await)
}

#[tauri::command]
async fn scan_dns_records(domain: String) -> Result<web_analyzer::domain_info::DnsInfo, AppError> {
    let d = domain.trim_start_matches("https://").trim_start_matches("http://").replace("www.", "");
    let clean = d.split('/').next().unwrap_or(&d).split(':').next().unwrap_or(&d).to_string();
    Ok(web_analyzer::domain_info::get_dns_records(&clean).await)
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
            db::get_db_stats,
            db::nuke_history,
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
            scan_geo_analysis,
            scan_ip_resolution,
            scan_whois,
            scan_ssl,
            scan_ports,
            scan_http_status,
            scan_security_headers,
            scan_dns_records,
            db::get_unique_scanned_domains
        ])
        .run(tauri::generate_context!())
        .expect("error while running WebQ application");
}

pub mod error;

use crate::error::AppError;
use web_analyzer::domain_info::{get_domain_info, DomainInfoResult};
use web_analyzer::domain_dns::{get_dns_records, DomainDnsResult};
use web_analyzer::seo_analysis::{analyze_advanced_seo, SeoAnalysisResult};
use web_analyzer::web_technologies::{detect_web_technologies, WebTechResult};
use web_analyzer::domain_validator::{validate_domains_bulk, BulkValidationResult};
use web_analyzer::subdomain_discovery::{discover_subdomains, SubdomainDiscoveryResult};
use web_analyzer::contact_spy::{crawl_contacts, ContactSpyResult};

#[tauri::command]
fn get_system_status() -> String {
    "WebQ Target Secured".into()
}

#[tauri::command]
async fn scan_domain_info(domain: String) -> Result<DomainInfoResult, AppError> {
    get_domain_info(&domain).await.map_err(|e| AppError::ModuleFailed(e.to_string()))
}

#[tauri::command]
async fn scan_domain_dns(domain: String) -> Result<DomainDnsResult, AppError> {
    get_dns_records(&domain).await.map_err(|e| AppError::ModuleFailed(e.to_string()))
}

#[tauri::command]
async fn scan_seo_analysis(url: String) -> Result<SeoAnalysisResult, AppError> {
    analyze_advanced_seo(&url).await.map_err(|e| AppError::ModuleFailed(e.to_string()))
}

#[tauri::command]
async fn scan_web_technologies(url: String) -> Result<WebTechResult, AppError> {
    detect_web_technologies(&url).await.map_err(|e| AppError::ModuleFailed(e.to_string()))
}

#[tauri::command]
async fn validate_bulk_domains(domains: Vec<String>) -> Result<BulkValidationResult, AppError> {
    // Concurrency is set to 10 to ensure stability on the user's local network for bulk scans
    Ok(validate_domains_bulk(&domains, 10).await)
}

#[tauri::command]
async fn scan_subdomains(domain: String) -> Result<SubdomainDiscoveryResult, AppError> {
    discover_subdomains(&domain).await.map_err(|e| AppError::ModuleFailed(e.to_string()))
}

#[tauri::command]
async fn scan_contacts(domain: String, max_pages: usize) -> Result<ContactSpyResult, AppError> {
    crawl_contacts(&domain, max_pages).await.map_err(|e| AppError::ModuleFailed(e.to_string()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_system_status,
            scan_domain_info,
            scan_domain_dns,
            scan_seo_analysis,
            scan_web_technologies,
            validate_bulk_domains,
            scan_subdomains,
            scan_contacts
        ])
        .run(tauri::generate_context!())
        .expect("error while running WebQ application");
}

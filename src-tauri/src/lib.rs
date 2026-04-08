pub mod error;

use crate::error::AppError;
use web_analyzer::domain_info::{get_domain_info, DomainInfoResult};

#[tauri::command]
fn get_system_status() -> String {
    "WebQ Target Secured".into()
}

#[tauri::command]
async fn scan_domain_info(domain: String) -> Result<DomainInfoResult, AppError> {
    get_domain_info(&domain).await.map_err(|e| AppError::ModuleFailed(e.to_string()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_system_status, scan_domain_info])
        .run(tauri::generate_context!())
        .expect("error while running WebQ application");
}

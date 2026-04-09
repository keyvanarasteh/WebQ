use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyStatus {
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn check_dependency(name: String) -> Result<DependencyStatus, String> {
    let mut cmd = Command::new(&name);
    
    // Some tools use -v, -V, or --version. We try to be smart.
    if name == "dig" {
        cmd.arg("-v");
    } else {
        cmd.arg("--version");
    }

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                
                // Nmap often prints to stdout, openssl to stdout/stderr
                let raw_version = if !stdout.is_empty() {
                    stdout
                } else {
                    stderr
                };

                // Extract just the first line for nicer UI
                let first_line = raw_version.lines().next().unwrap_or("Unknown version").to_string();

                Ok(DependencyStatus {
                    name,
                    installed: true,
                    version: Some(first_line),
                    error: None,
                })
            } else {
                Ok(DependencyStatus {
                    name,
                    installed: false,
                    version: None,
                    error: Some("Command executed but returned an error status".to_string()),
                })
            }
        }
        Err(e) => Ok(DependencyStatus {
            name,
            installed: false,
            version: None,
            error: Some(e.to_string()),
        }),
    }
}

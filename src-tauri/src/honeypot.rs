use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    response::IntoResponse,
    routing::any,
    Router,
};
use chrono::Utc;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::{Mutex, RwLock};
use web_analyzer::react_honeypot::{HoneypotConfig, HoneypotEngine, RawRequest, AttackerProfile};
use crate::error::AppError;

pub struct HoneypotState {
    pub engine: Arc<RwLock<HoneypotEngine>>,
    pub server_task: Mutex<Option<tokio::task::JoinHandle<()>>>,
    pub app_handle: AppHandle,
}

impl HoneypotState {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            engine: Arc::new(RwLock::new(HoneypotEngine::new())),
            server_task: Mutex::new(None),
            app_handle,
        }
    }
}

async fn honeypot_handler(
    State(state): State<Arc<HoneypotState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
) -> impl IntoResponse {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let query_string = req.uri().query().unwrap_or("").to_string();
    
    let mut headers = HashMap::new();
    for (k, v) in req.headers() {
        if let Ok(value) = v.to_str() {
            headers.insert(k.to_string(), value.to_string());
        }
    }
    
    let ip = addr.ip().to_string();
    
    // Read body (this is a bit tricky with axum since we need the bytes without failing)
    // For simplicity, we use hyper body reading or axum bytes extractor
    // We will use hyper's body conversion
    let body_bytes = axum::body::to_bytes(req.into_body(), 1024 * 1024).await.unwrap_or_default();
    let body = String::from_utf8_lossy(&body_bytes).to_string();

    let raw_request = RawRequest {
        method,
        path,
        query_string,
        body,
        headers,
        ip,
        timestamp: Utc::now(),
    };

    let mut engine = state.engine.write().await;
    let result = engine.process_request(&raw_request);

    // Emit event if any attack was detected
    for detection in &result.detections {
        let _ = state.app_handle.emit("honeypot-attack-detected", detection);
    }

    // Build the response based on the simulation
    let status = StatusCode::from_u16(result.simulated_status).unwrap_or(StatusCode::OK);
    
    // Apply content type header
    let response = axum::response::Response::builder()
        .status(status)
        .header("Content-Type", result.content_type);
    
    if result.suggested_delay_ms > 0 {
        tokio::time::sleep(tokio::time::Duration::from_millis(result.suggested_delay_ms)).await;
    }
    
    response.body(axum::body::Body::from(result.simulated_body)).unwrap()
}

#[tauri::command]
pub async fn start_honeypot(port: u16, state: tauri::State<'_, Arc<HoneypotState>>) -> Result<String, AppError> {
    let mut server_task = state.server_task.lock().await;
    if server_task.is_some() {
        return Err(AppError::ModuleFailed("Honeypot server is already running".to_string()));
    }

    let shared_state = state.inner().clone();
    
    let app = Router::new()
        .route("/", any(honeypot_handler))
        .route("/*path", any(honeypot_handler))
        .with_state(shared_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| AppError::ModuleFailed(format!("Failed to bind to port {}: {}", port, e)))?;

    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await;
    });

    *server_task = Some(handle);

    Ok(format!("Honeypot started on port {}", port))
}

#[tauri::command]
pub async fn stop_honeypot(state: tauri::State<'_, Arc<HoneypotState>>) -> Result<String, AppError> {
    let mut server_task = state.server_task.lock().await;
    if let Some(handle) = server_task.take() {
        handle.abort();
        Ok("Honeypot stopped".to_string())
    } else {
        Err(AppError::ModuleFailed("Honeypot server is not running".to_string()))
    }
}

#[tauri::command]
pub async fn get_honeypot_status(state: tauri::State<'_, Arc<HoneypotState>>) -> Result<bool, AppError> {
    let server_task = state.server_task.lock().await;
    Ok(server_task.is_some())
}

#[tauri::command]
pub async fn get_top_attackers(state: tauri::State<'_, Arc<HoneypotState>>) -> Result<Vec<AttackerProfile>, AppError> {
    let engine = state.engine.read().await;
    let threats = engine.get_top_threats(50);
    Ok(threats.into_iter().cloned().collect())
}

#[derive(serde::Serialize)]
pub struct LocalTestResult {
    pub detections: Vec<web_analyzer::react_honeypot::AttackEvent>,
    pub should_block: bool,
    pub simulated_status: u16,
    pub suggested_delay_ms: u64,
}

#[tauri::command]
pub async fn test_payload_locally(
    method: String,
    path: String,
    query_string: String,
    body: String,
    headers: HashMap<String, String>,
    state: tauri::State<'_, Arc<HoneypotState>>
) -> Result<LocalTestResult, AppError> {
    let raw_request = RawRequest {
        method,
        path,
        query_string,
        body,
        headers,
        ip: "127.0.0.1".to_string(),
        timestamp: Utc::now(),
    };

    let mut engine = state.engine.write().await;
    let result = engine.process_request(&raw_request);

    // Also emit so the UI logs it
    for detection in &result.detections {
        let _ = state.app_handle.emit("honeypot-attack-detected", detection);
    }

    Ok(LocalTestResult {
        detections: result.detections,
        should_block: result.should_block,
        simulated_status: result.simulated_status,
        suggested_delay_ms: result.suggested_delay_ms,
    })
}

#[tauri::command]
pub async fn update_honeypot_config(
    realistic_timing: bool,
    fake_rsc_responses: bool,
    session_tracking: bool,
    progressive_sizing: bool,
    state: tauri::State<'_, Arc<HoneypotState>>
) -> Result<(), AppError> {
    let mut engine = state.engine.write().await;
    let config = HoneypotConfig {
        realistic_timing,
        fake_rsc_responses,
        session_tracking,
        progressive_sizing,
        ..Default::default()
    };
    
    // Create new engine with updated config but try to retain state if possible
    // (Since we can't easily swap config without a new engine, let's just create a new one)
    *engine = HoneypotEngine::with_config(config);
    Ok(())
}

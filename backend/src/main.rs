use axum::{
    extract::{State, Json},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{oneshot, RwLock};
use std::collections::HashMap;

mod config;
mod context;
mod gate;
mod listdir;
mod queue;
mod readfile;
mod registry;
mod runcommand;
mod search;
mod tool;
mod writefile;

use crate::config::Config;
use crate::context::ContextBuilder;
use crate::gate::Gate;
use crate::queue::Loop;
use crate::readfile::Readfile;
use crate::registry::ToolRegistry;
use crate::runcommand::Runcommand;
use crate::search::Search;
use crate::writefile::Writefile;
use crate::listdir::Listdir;

#[derive(Debug, Deserialize)]
struct ChatRequest {
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct CallRequest {
    tool: String,
    input: Value,
}

#[derive(Debug, Deserialize)]
struct ContextRequest {
    task: String,
}

#[derive(Debug, Deserialize)]
struct ApproveRequest {
    request_id: String,
    approved: bool,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
}

#[derive(Debug, Serialize)]
struct CallResponse {
    result: Value,
}

#[derive(Debug, Serialize)]
struct ContextResponse {
    context: Vec<crate::context::ContextItem>,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Clone)]
struct AppState {
    registry: Arc<ToolRegistry>,
    gate: Arc<Gate>,
    pending_approvals: Arc<RwLock<HashMap<String, oneshot::Sender<bool>>>>,
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

async fn chat_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Json<ChatResponse> {
    let registry = (*state.registry).clone();
    let config = Config::from_env();
    let gate = state.gate.clone();
    let pending = state.pending_approvals.clone();

    let loop_handler = Loop::new(registry, config, gate, pending);

    let request_value = serde_json::json!({
        "prompt": payload.prompt,
    });

    let response_value = loop_handler.run(request_value).await;
    let response_text = response_value.get("messages")
        .and_then(Value::as_array)
        .and_then(|arr| arr.last())
        .and_then(|msg| msg.get("content"))
        .and_then(Value::as_str)
        .unwrap_or("no response")
        .to_string();

    Json(ChatResponse {
        response: response_text,
    })
}

async fn call_handler(
    State(state): State<AppState>,
    Json(payload): Json<CallRequest>,
) -> Result<Json<CallResponse>, StatusCode> {
    let tool = state.registry.get(&payload.tool).ok_or(StatusCode::NOT_FOUND)?;
    let result = tool.execute(payload.input).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(CallResponse { result }))
}

async fn context_handler(
    State(_state): State<AppState>,
    Json(payload): Json<ContextRequest>,
) -> Json<ContextResponse> {
    let config = Config::from_env();
    let builder = ContextBuilder::new(config);
    let items = builder.build(&payload.task).unwrap_or_default();
    Json(ContextResponse { context: items })
}

async fn approve_handler(
    State(state): State<AppState>,
    Json(req): Json<ApproveRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut guard = state.pending_approvals.write().await;
    if let Some(tx) = guard.remove(&req.request_id) {
        let _ = tx.send(req.approved);
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let mut registry = ToolRegistry::new();

    registry.register(Arc::new(Readfile)).unwrap();
    registry.register(Arc::new(Search::new(config.clone()))).unwrap();
    registry.register(Arc::new(Writefile::new(config.clone()))).unwrap();
    registry.register(Arc::new(Runcommand::new(config.clone()))).unwrap();
    registry.register(Arc::new(Listdir::new(config.clone()))).unwrap();

    let app_state = AppState {
        registry: Arc::new(registry),
        gate: Arc::new(Gate::new(config.clone())),
        pending_approvals: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/chat", post(chat_handler))
        .route("/call", post(call_handler))
        .route("/context", post(context_handler))
        .route("/approve", post(approve_handler))
        .with_state(app_state);

    let addr = config.server_addr();
    println!("Bandhu backend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

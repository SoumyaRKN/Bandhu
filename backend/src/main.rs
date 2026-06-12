use axum::{
    extract::{Json, State},
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    Router,
};
use env_logger;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{oneshot, RwLock};
use tower_http::cors::{Any, CorsLayer};

mod applypatch;
mod config;
mod context;
mod diff;
mod error;
mod gate;
mod listdir;
mod model;
mod queue;
mod readfile;
mod registry;
mod runcommand;
mod search;
mod tool;
mod writefile;

use crate::applypatch::Applypatch;
use crate::config::Config;
use crate::context::ContextBuilder;
use crate::error::BackendError;
use crate::gate::Gate;
use crate::listdir::Listdir;
use crate::queue::Loop;
use crate::readfile::Readfile;
use crate::registry::ToolRegistry;
use crate::runcommand::Runcommand;
use crate::search::Search;
use crate::writefile::Writefile;

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
    messages: Vec<Value>,
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
    pending_writes: Arc<RwLock<HashMap<String, Value>>>,
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
    let pending_writes = state.pending_writes.clone();

    let loop_handler = Loop::new(registry, config, gate, pending, pending_writes);

    let request_value = serde_json::json!({
        "prompt": payload.prompt,
    });

    let response_value = loop_handler.run(request_value).await;
    let messages = response_value
        .get("messages")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let response_text = response_value
        .get("messages")
        .and_then(Value::as_array)
        .and_then(|arr| arr.last())
        .and_then(|msg| msg.get("content"))
        .and_then(Value::as_str)
        .unwrap_or("no response")
        .to_string();

    Json(ChatResponse {
        response: response_text,
        messages,
    })
}

async fn call_handler(
    State(state): State<AppState>,
    Json(payload): Json<CallRequest>,
) -> Result<Json<CallResponse>, StatusCode> {
    let config = Config::from_env();
    validatecallinput(&config, &state.registry, &payload.tool, &payload.input).map_err(|e| {
        log::warn!(
            "call validation failed: tool='{}', error={}",
            payload.tool,
            e
        );
        StatusCode::BAD_REQUEST
    })?;

    let tool = state
        .registry
        .get(&payload.tool)
        .ok_or(StatusCode::NOT_FOUND)?;
    let result = tool
        .execute(payload.input)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(CallResponse { result }))
}

fn validatecallinput(
    config: &Config,
    registry: &ToolRegistry,
    tool: &str,
    input: &Value,
) -> Result<(), BackendError> {
    let size = serde_json::to_string(input)
        .map_err(|e| BackendError::Parse(e.to_string()))?
        .len();
    if size > config.tool_input_limit {
        return Err(BackendError::Tool(format!(
            "input exceeds {} bytes",
            config.tool_input_limit
        )));
    }

    if !config.schema_validate {
        return Ok(());
    }

    registry
        .validate(tool, input)
        .map_err(|e| BackendError::Tool(e.to_string()))
}

fn cors(config: &Config) -> CorsLayer {
    let layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    if config.cors.iter().any(|origin| origin == "*") {
        return layer.allow_origin(Any);
    }

    let origins = config
        .cors
        .iter()
        .filter_map(|origin| origin.parse::<HeaderValue>().ok())
        .collect::<Vec<_>>();

    if origins.is_empty() {
        layer.allow_origin(Any)
    } else {
        layer.allow_origin(origins)
    }
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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let config = Config::from_env();
    let mut registry = ToolRegistry::new();

    registry.register(Arc::new(Readfile)).unwrap();
    registry
        .register(Arc::new(Search::new(config.clone())))
        .unwrap();
    registry
        .register(Arc::new(Writefile::new(config.clone())))
        .unwrap();
    registry
        .register(Arc::new(Applypatch::new(config.clone())))
        .unwrap();
    registry
        .register(Arc::new(Runcommand::new(config.clone())))
        .unwrap();
    registry
        .register(Arc::new(Listdir::new(config.clone())))
        .unwrap();

    let app_state = AppState {
        registry: Arc::new(registry),
        gate: Arc::new(Gate::new(config.clone())),
        pending_approvals: Arc::new(RwLock::new(HashMap::new())),
        pending_writes: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/chat", post(chat_handler))
        .route("/call", post(call_handler))
        .route("/context", post(context_handler))
        .route("/approve", post(approve_handler))
        .with_state(app_state)
        .layer(cors(&config));

    let addr = config.server_addr();
    println!("Bandhu backend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

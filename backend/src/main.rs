use crate::queue::Loop;
use crate::readfile::Readfile;
use crate::registry::ToolRegistry;
use crate::search::Search;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;

mod queue;
mod readfile;
mod registry;
mod search;
mod tool;

#[derive(Debug, Deserialize)]
struct ChatRequest {
    prompt: String,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
}

async fn chat_handler(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    let registry = tools();
    let loop_handler = Loop::new(registry);
    
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

fn tools() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    
    registry.register(Arc::new(Readfile)).unwrap();
    registry.register(Arc::new(Search)).unwrap();
    
    registry
}

#[tokio::main]
async fn main() {
    let registry = tools();
    let app = Router::new()
        .with_state(registry)
        .route("/chat", post(chat_handler));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Bandhu backend listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod registry;
mod tool;

#[derive(Debug, Deserialize)]
struct ChatRequest {
    prompt: String,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

async fn chat_handler(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    let ollama_request = OllamaRequest {
        model: "qwen2.5-coder:7b".to_string(),
        prompt: payload.prompt,
        stream: false,
    };

    let response_text = match call_ollama(ollama_request).await {
        Ok(resp) => resp,
        Err(e) => format!("Error calling Ollama: {}", e),
    };

    Json(ChatResponse {
        response: response_text,
    })
}

async fn call_ollama(req: OllamaRequest) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&req)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: OllamaResponse = response.json().await.map_err(|e| e.to_string())?;

    Ok(body.response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/chat", post(chat_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Bandhu backend listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

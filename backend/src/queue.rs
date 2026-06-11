use crate::config::Config;
use crate::gate::{Gate, ApprovalRequest};
use crate::registry::ToolRegistry;
use crate::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::{oneshot, RwLock};
use std::collections::HashMap;

pub struct Loop {
    registry: ToolRegistry,
    model: Model,
    config: Config,
    gate: Arc<Gate>,
    pending: Arc<RwLock<HashMap<String, oneshot::Sender<bool>>>>,
}

impl Loop {
    pub fn new(
        registry: ToolRegistry,
        config: Config,
        gate: Arc<Gate>,
        pending: Arc<RwLock<HashMap<String, oneshot::Sender<bool>>>>,
    ) -> Self {
        Self {
            model: Model::new(config.clone()),
            registry,
            config,
            gate,
            pending,
        }
    }

    pub async fn run(&self, request: Value) -> Value {
        let prompt = request.get("prompt").and_then(Value::as_str).unwrap_or("").to_string();
        let mut context = request.get("context").cloned().unwrap_or(Value::Null);
        
        let mut messages = vec![];
        let max_iterations = self.config.max_iterations;
        
        for iteration in 1..=max_iterations {
            let full_prompt = self.build_prompt(&prompt, &context);
            let output = self.model.call(full_prompt).await;
            
            if let Some(tool_call) = self.parse_tool_call(&output) {
                let tool_id = &tool_call.id;
                let tool = self.registry.get(tool_id);
                
                match tool {
                    Some(tool) => {
                        if tool.requires() {
                            if let Err(e) = self.gate.check(&tool_call.input, tool_id) {
                                messages.push(json!({
                                    "type": "tool_error",
                                    "tool": tool_id,
                                    "error": e
                                }));
                                continue;
                            }
                            
                            if self.gate.requires_approval(tool_id) {
                                let req_id = format!("{}-{}", tool_id, iteration);
                                let (tx, rx) = oneshot::channel();
                                {
                                    let mut guard = self.pending.write().await;
                                    guard.insert(req_id.clone(), tx);
                                }
                                
                                let approval_msg = json!({
                                    "type": "tool_approval",
                                    "id": req_id,
                                    "tool": tool_id,
                                    "input": tool_call.input
                                });
                                messages.push(approval_msg);
                                
                                context = append_context(&context, json!({
                                    "type": "tool_approval",
                                    "id": req_id,
                                    "tool": tool_id
                                }));
                                
                                match rx.await {
                                    Ok(true) => {
                                        let result = tool.execute(tool_call.input);
                                        let result_value = match result {
                                            Ok(v) => json!({
                                                "type": "tool_result",
                                                "id": req_id,
                                                "result": v
                                            }),
                                            Err(e) => json!({
                                                "type": "tool_error",
                                                "id": req_id,
                                                "error": e
                                            }),
                                        };
                                        context = append_context(&context, result_value);
                                    }
                                    Ok(false) => {
                                        let reject_msg = json!({
                                            "type": "tool_error",
                                            "id": req_id,
                                            "error": "rejected by user"
                                        });
                                        context = append_context(&context, reject_msg);
                                    }
                                    Err(_) => {
                                        let timeout_msg = json!({
                                            "type": "tool_error",
                                            "id": req_id,
                                            "error": "approval timeout"
                                        });
                                        context = append_context(&context, timeout_msg);
                                    }
                                }
                                continue;
                            }
                        }
                        
                        let result = tool.execute(tool_call.input);
                        let result_value = match result {
                            Ok(v) => json!({
                                "type": "tool_result",
                                "id": tool_id,
                                "result": v
                            }),
                            Err(e) => json!({
                                "type": "tool_error",
                                "id": tool_id,
                                "error": e
                            }),
                        };
                        context = append_context(&context, result_value);
                        continue;
                    }
                    None => {
                        messages.push(json!({
                            "type": "error",
                            "error": format!("tool not found: {}", tool_id)
                        }));
                    }
                }
            }
            
            messages.push(json!({
                "type": "response",
                "content": output
            }));
            
            return json!({
                "type": "complete",
                "messages": messages,
                "iterations": iteration
            });
        }
        
        messages.push(json!({
            "type": "error",
            "error": "max iterations reached"
        }));
        
        json!({
            "type": "complete",
            "messages": messages,
            "iterations": max_iterations
        })
    }
    
    fn build_prompt(&self, prompt: &str, context: &Value) -> String {
        let tools = self.registry.ids();
        let tools_json: Vec<Value> = tools.iter().filter_map(|id| {
            self.registry.get(id).map(|tool| {
                json!({
                    "id": tool.id(),
                    "name": tool.name(),
                    "description": tool.desc(),
                    "schema": tool.schema()
                })
            })
        }).collect();
        
        let tools_str = serde_json::to_string_pretty(&tools_json).unwrap_or_else(|_| "[]".to_string());
        let context_str = context_to_string(context);
        
        let template = std::env::var("BANDHU_PROMPT_TEMPLATE")
            .unwrap_or_else(|_| {
                "Available tools:\n{}\n\nContext:\n{}\n\nTask: {}".to_string()
            });
        
        let mut result = template;
        result = result.replacen("{}", &tools_str, 1);
        result = result.replacen("{}", &context_str, 1);
        result = result.replacen("{}", prompt, 1);
        result
    }
    
    fn parse_tool_call(&self, output: &str) -> Option<ToolCall> {
        let trimmed = output.trim();
        if !trimmed.starts_with('{') {
            return None;
        }
        
        let json: Value = serde_json::from_str(trimmed).ok()?;
        
        let id = json.get("tool")?.as_str()?.to_string();
        let input = json.get("input")?.clone();
        
        Some(ToolCall { id, input })
    }
}

struct ToolCall {
    id: String,
    input: Value,
}

struct Model {
    config: Config,
}

impl Model {
    fn new(config: Config) -> Self {
        Self { config }
    }
    
    async fn call(&self, prompt: String) -> String {
        let client = reqwest::Client::new();
        let request = OllamaRequest {
            model: self.config.ollama_model.clone(),
            prompt,
            stream: self.config.ollama_stream,
        };
        
        match client
            .post(self.config.ollama_api_url())
            .json(&request)
            .send()
            .await
        {
            Ok(response) => {
                match response.json::<OllamaResponse>().await {
                    Ok(resp) => resp.response,
                    Err(_) => "error parsing ollama response".to_string(),
                }
            }
            Err(e) => format!("error calling ollama: {}", e),
        }
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

fn context_to_string(context: &Value) -> String {
    match context {
        Value::String(s) => s.clone(),
        Value::Object(obj) => {
            obj.iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        }
        Value::Array(arr) => {
            arr.iter()
                .map(context_to_string)
                .collect::<Vec<_>>()
                .join("\n")
        }
        _ => String::new(),
    }
}

fn append_context(context: &Value, new: Value) -> Value {
    match context {
        Value::Null => json!([new]),
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(new);
            json!(new_arr)
        }
        _ => json!([context.clone(), new]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn builds_prompt() {
        let registry = ToolRegistry::new();
        let config = Config::from_env();
        let gate = Arc::new(Gate::new(config.clone()));
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending);
        
        let prompt = "test task";
        let context = Value::Null;
        
        let built = loop_handler.build_prompt(prompt, &context);
        
        assert!(built.contains("Available tools:"));
        assert!(built.contains("test task"));
    }
    
    #[test]
    fn parses_tool_call_json() {
        let registry = ToolRegistry::new();
        let config = Config::from_env();
        let gate = Arc::new(Gate::new(config.clone()));
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending);
        
        let output = r#"{"tool": "readfile", "input": {"path": "/test.rs"}}"#;
        
        let tool_call = loop_handler.parse_tool_call(output);
        
        assert!(tool_call.is_some());
        let tc = tool_call.unwrap();
        assert_eq!(tc.id, "readfile");
    }
    
    #[test]
    fn rejects_non_json() {
        let registry = ToolRegistry::new();
        let config = Config::from_env();
        let gate = Arc::new(Gate::new(config.clone()));
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending);
        
        let output = "just a regular response without tool call";
        
        let tool_call = loop_handler.parse_tool_call(output);
        
        assert!(tool_call.is_none());
    }
}

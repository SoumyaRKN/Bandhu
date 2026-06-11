use crate::registry::ToolRegistry;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const MAX_ITERATIONS: usize = 10;

pub struct Loop {
    model: Model,
    registry: ToolRegistry,
}

impl Loop {
    pub fn new(registry: ToolRegistry) -> Self {
        Self {
            model: Model::new(),
            registry,
        }
    }

    pub async fn run(&self, request: Value) -> Value {
        let prompt = request.get("prompt").and_then(Value::as_str).unwrap_or("").to_string();
        let mut context = request.get("context").cloned().unwrap_or(Value::Null);
        
        let mut messages = vec![];
        let mut iterations = 0;
        
        while iterations < MAX_ITERATIONS {
            iterations += 1;
            
            let full_prompt = self.build_prompt(&prompt, &context);
            let output = self.model.call(full_prompt).await;
            
            if let Some(tool_call) = self.parse_tool_call(&output) {
                let tool = self.registry.get(&tool_call.id);
                
                if let Some(tool) = tool {
                    let result = tool.execute(tool_call.input);
                    
                    let result_value = match result {
                        Ok(v) => json!({
                            "type": "tool_result",
                            "id": tool_call.id,
                            "result": v
                        }),
                        Err(e) => json!({
                            "type": "tool_error",
                            "id": tool_call.id,
                            "error": e
                        }),
                    };
                    
                    context = append_context(&context, result_value);
                    continue;
                }
                
                messages.push(json!({
                    "type": "error",
                    "error": format!("tool not found: {}", tool_call.id)
                }));
            }
            
            messages.push(json!({
                "type": "response",
                "content": output
            }));
            
            return json!({
                "type": "complete",
                "messages": messages,
                "iterations": iterations
            });
        }
        
        messages.push(json!({
            "type": "error",
            "error": "max iterations reached"
        }));
        
        json!({
            "type": "complete",
            "messages": messages,
            "iterations": iterations
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
        
        format!(
            r#"Available tools:
{}

Context:
{}

Task: {}"#,
            tools_str,
            context_to_string(context),
            prompt
        )
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

struct Model;

impl Model {
    fn new() -> Self {
        Self
    }
    
    async fn call(&self, model: String, prompt: String) -> String {
        let client = reqwest::Client::new();
        let request = OllamaRequest {
            model,
            prompt,
            stream: false,
        };
        
        match client
            .post("http://localhost:11434/api/generate")
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
        let loop_handler = Loop::new(registry);
        
        let prompt = "test task";
        let context = Value::Null;
        
        let built = loop_handler.build_prompt(prompt, &context);
        
        assert!(built.contains("Available tools:"));
        assert!(built.contains("test task"));
    }
    
    #[test]
    fn parses_tool_call_json() {
        let registry = ToolRegistry::new();
        let loop_handler = Loop::new(registry);
        
        let output = r#"{"tool": "readfile", "input": {"path": "/test.rs"}}"#;
        
        let tool_call = loop_handler.parse_tool_call(output);
        
        assert!(tool_call.is_some());
        let tc = tool_call.unwrap();
        assert_eq!(tc.id, "readfile");
    }
    
    #[test]
    fn rejects_non_json() {
        let registry = ToolRegistry::new();
        let loop_handler = Loop::new(registry);
        
        let output = "just a regular response without tool call";
        
        let tool_call = loop_handler.parse_tool_call(output);
        
        assert!(tool_call.is_none());
    }
}
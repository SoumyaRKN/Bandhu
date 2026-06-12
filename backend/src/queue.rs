use crate::config::Config;
use crate::context::ContextBuilder;
use crate::error::{BackendError, BackendResult};
use crate::gate::Gate;
use crate::model::Model;
use crate::registry::ToolRegistry;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{oneshot, RwLock};

pub struct Loop {
    registry: ToolRegistry,
    model: Model,
    config: Config,
    gate: Arc<Gate>,
    pending: Arc<RwLock<HashMap<String, oneshot::Sender<bool>>>>,
    pending_writes: Arc<RwLock<HashMap<String, Value>>>,
}

impl Loop {
    pub fn new(
        registry: ToolRegistry,
        config: Config,
        gate: Arc<Gate>,
        pending: Arc<RwLock<HashMap<String, oneshot::Sender<bool>>>>,
        pending_writes: Arc<RwLock<HashMap<String, Value>>>,
    ) -> Self {
        Self {
            model: Model::new(config.clone()),
            registry,
            config,
            gate,
            pending,
            pending_writes,
        }
    }

    pub async fn run(&self, request: Value) -> Value {
        let prompt = request
            .get("prompt")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let mut context = request.get("context").cloned().unwrap_or_else(|| {
            let builder = ContextBuilder::new(self.config.clone());
            let items = builder.build(&prompt).unwrap_or_default();
            items
                .into_iter()
                .map(|item| {
                    json!({
                        "path": item.path,
                        "content": item.content
                    })
                })
                .collect::<Vec<_>>()
                .into()
        });

        let mut messages = vec![];
        let max_iterations = self.config.max_iterations;
        log::info!(
            "loop start: prompt='{}', max_iterations={}",
            prompt,
            max_iterations
        );

        for iteration in 1..=max_iterations {
            log::info!("loop iteration {}/{}", iteration, max_iterations);
            let full_prompt = self.build_prompt(&prompt, &context);
            log::debug!("prompt built ({} chars)", full_prompt.len());
            let output = self.model.call(full_prompt).await;
            log::debug!("model output ({} chars): {}", output.len(), output);

            if let Some(tool_call) = self.parse_tool_call(&output) {
                let tool_id = &tool_call.id;
                let tool = self.registry.get(tool_id);
                log::info!(
                    "tool call detected: id='{}', found={}",
                    tool_id,
                    tool.is_some()
                );

                match tool {
                    Some(tool) => {
                        if let Err(e) = self.validateinput(tool_id, &tool_call.input) {
                            log::error!("tool validation failed: tool='{}', error={}", tool_id, e);
                            messages.push(json!({
                                "type": "tool_error",
                                "tool": tool_id,
                                "error": e.to_string()
                            }));
                            continue;
                        }

                        if tool.requires() {
                            if let Err(e) = self.gate.check(&tool_call.input, tool_id) {
                                log::error!("safety filter rejected tool '{}': {}", tool_id, e);
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

                                let tool_input = tool_call.input.clone();

                                let diff = if tool_id == "writefile" {
                                    let diff = crate::writefile::Writefile::diff(&tool_input)
                                        .unwrap_or_default();
                                    Some(diff)
                                } else {
                                    None
                                };

                                let approval_msg = json!({
                                    "type": "tool_approval",
                                    "id": req_id,
                                    "tool": tool_id,
                                    "input": tool_input.clone(),
                                    "diff": diff
                                });
                                messages.push(approval_msg.clone());
                                log::info!(
                                    "approval requested: id='{}', tool='{}'",
                                    req_id,
                                    tool_id
                                );

                                if tool_id == "writefile" {
                                    {
                                        let mut guard = self.pending_writes.write().await;
                                        guard.insert(req_id.clone(), tool_input.clone());
                                    }
                                }

                                context = append_context(
                                    &context,
                                    json!({
                                        "type": "tool_approval",
                                        "id": req_id,
                                        "tool": tool_id
                                    }),
                                );

                                match rx.await {
                                    Ok(true) => {
                                        log::info!("approval granted: id='{}'", req_id);
                                        let stored_input = if tool_id == "writefile" {
                                            let guard = self.pending_writes.read().await;
                                            guard
                                                .get(&req_id)
                                                .cloned()
                                                .unwrap_or_else(|| tool_input.clone())
                                        } else {
                                            tool_input
                                        };

                                        log::debug!(
                                            "executing tool '{}' with input: {:?}",
                                            tool_id,
                                            stored_input
                                        );
                                        let result = tool.execute(stored_input);
                                        let result_value = match result {
                                            Ok(v) => {
                                                log::info!(
                                                    "tool '{}' executed successfully",
                                                    tool_id
                                                );
                                                json!({
                                                    "type": "tool_result",
                                                    "id": req_id,
                                                    "result": v
                                                })
                                            }
                                            Err(e) => {
                                                log::error!(
                                                    "tool '{}' execution failed: {}",
                                                    tool_id,
                                                    e
                                                );
                                                json!({
                                                    "type": "tool_error",
                                                    "id": req_id,
                                                    "error": e
                                                })
                                            }
                                        };

                                        if tool_id == "writefile" {
                                            let mut guard = self.pending_writes.write().await;
                                            guard.remove(&req_id);
                                        }

                                        context = append_context(&context, result_value);
                                    }
                                    Ok(false) => {
                                        log::info!("approval rejected: id='{}'", req_id);
                                        let reject_msg = json!({
                                            "type": "tool_error",
                                            "id": req_id,
                                            "error": "rejected by user"
                                        });
                                        context = append_context(&context, reject_msg);
                                    }
                                    Err(_) => {
                                        log::warn!("approval timeout: id='{}'", req_id);
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

                        log::debug!(
                            "executing tool '{}' with input: {:?}",
                            tool_id,
                            tool_call.input
                        );
                        let result = tool.execute(tool_call.input);
                        let result_value = match result {
                            Ok(v) => {
                                log::info!("tool '{}' executed successfully", tool_id);
                                json!({
                                    "type": "tool_result",
                                    "id": tool_id,
                                    "result": v
                                })
                            }
                            Err(e) => {
                                log::error!("tool '{}' execution failed: {}", tool_id, e);
                                json!({
                                    "type": "tool_error",
                                    "id": tool_id,
                                    "error": e
                                })
                            }
                        };
                        context = append_context(&context, result_value);
                        continue;
                    }
                    None => {
                        log::warn!("tool not found: '{}'", tool_id);
                        messages.push(json!({
                            "type": "error",
                            "error": format!("tool not found: {}", tool_id)
                        }));
                    }
                }
            }

            log::info!("final response received ({} chars)", output.len());
            messages.push(json!({
                "type": "response",
                "content": output
            }));

            log::info!("loop complete after {} iterations", iteration);
            return json!({
                "type": "complete",
                "messages": messages,
                "iterations": iteration
            });
        }

        log::warn!("max iterations reached: {}", max_iterations);
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

    fn validateinput(&self, tool_id: &str, input: &Value) -> BackendResult<()> {
        let size = serde_json::to_string(input)
            .map_err(|e| BackendError::Parse(e.to_string()))?
            .len();
        if size > self.config.tool_input_limit {
            return Err(BackendError::Tool(format!(
                "input exceeds {} bytes",
                self.config.tool_input_limit
            )));
        }

        if !self.config.schema_validate {
            return Ok(());
        }

        self.registry
            .validate(tool_id, input)
            .map_err(|e| BackendError::Tool(e.to_string()))
    }

    fn build_prompt(&self, prompt: &str, context: &Value) -> String {
        let tools = self.registry.ids();
        let tools_json: Vec<Value> = tools
            .iter()
            .filter_map(|id| {
                self.registry.get(id).map(|tool| {
                    json!({
                        "id": tool.id(),
                        "name": tool.name(),
                        "description": tool.desc(),
                        "schema": tool.schema()
                    })
                })
            })
            .collect();

        let tools_str =
            serde_json::to_string_pretty(&tools_json).unwrap_or_else(|_| "[]".to_string());
        let context_str = context_to_string(context);

        let template = std::env::var("BANDHU_PROMPT_TEMPLATE")
            .unwrap_or_else(|_| "Available tools:\n{}\n\nContext:\n{}\n\nTask: {}".to_string());

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

fn context_to_string(context: &Value) -> String {
    match context {
        Value::String(s) => s.clone(),
        Value::Object(obj) => obj
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n"),
        Value::Array(arr) => arr
            .iter()
            .map(context_to_string)
            .collect::<Vec<_>>()
            .join("\n"),
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
    use crate::readfile::Readfile;
    use env_logger;
    use tokio::runtime::Runtime;

    #[test]
    fn builds_prompt() {
        let registry = ToolRegistry::new();
        let config = Config::from_env();
        let gate = Arc::new(Gate::new(config.clone()));
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let pending_writes = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending, pending_writes);

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
        let pending_writes = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending, pending_writes);

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
        let pending_writes = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending, pending_writes);

        let output = "just a regular response without tool call";

        let tool_call = loop_handler.parse_tool_call(output);

        assert!(tool_call.is_none());
    }

    #[test]
    fn validatesinput() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(Readfile)).unwrap();
        let config = Config::from_env();
        let gate = Arc::new(Gate::new(config.clone()));
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let pending_writes = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending, pending_writes);

        let result = loop_handler.validateinput("readfile", &json!({"path": ""}));

        assert!(result.is_err());
    }

    #[test]
    fn skipsschema() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(Readfile)).unwrap();
        let mut config = Config::from_env();
        config.schema_validate = false;
        let gate = Arc::new(Gate::new(config.clone()));
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let pending_writes = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending, pending_writes);

        let result = loop_handler.validateinput("readfile", &json!({"path": ""}));

        assert!(result.is_ok());
    }

    #[test]
    fn rejectslargeinput() {
        let registry = ToolRegistry::new();
        let mut config = Config::from_env();
        config.tool_input_limit = 8;
        let gate = Arc::new(Gate::new(config.clone()));
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let pending_writes = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending, pending_writes);

        let result = loop_handler.validateinput("readfile", &json!({"path": "toolong"}));

        assert!(result.is_err());
    }

    #[test]
    fn loop_runs_with_logging() {
        let _ = env_logger::try_init();
        let registry = ToolRegistry::new();
        let config = Config::from_env();
        let gate = Arc::new(Gate::new(config.clone()));
        let pending = Arc::new(RwLock::new(HashMap::new()));
        let pending_writes = Arc::new(RwLock::new(HashMap::new()));
        let loop_handler = Loop::new(registry, config, gate, pending, pending_writes);

        let request = json!({
            "prompt": "what is 2+2?",
            "context": Value::Null
        });

        let rt = Runtime::new().unwrap();
        let result = rt.block_on(loop_handler.run(request));

        assert_eq!(
            result.get("type").and_then(|v| v.as_str()),
            Some("complete")
        );
        assert!(result.get("messages").and_then(|v| v.as_array()).is_some());
    }
}

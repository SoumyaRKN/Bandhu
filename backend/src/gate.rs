use crate::config::Config;
use serde_json::Value;
use std::path::PathBuf;

pub struct Gate {
    config: Config,
}

impl Gate {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn check(&self, tool_input: &Value, tool_id: &str) -> Result<(), String> {
        if tool_id == "runcommand" {
            let Some(command) = tool_input.get("command").and_then(|v| v.as_str()) else {
                return Err("missing command".to_string());
            };
            let lower = command.to_lowercase();
            for pattern in &self.config.forbidden_command_patterns {
                if lower.contains(pattern) || lower == *pattern {
                    return Err(format!("forbidden command pattern: {}", pattern));
                }
            }
        }

        if tool_id == "writefile" || tool_id == "readfile" || tool_id == "runcommand" {
            let Some(path_val) = tool_input.get("path").and_then(|v| v.as_str()) else {
                return Ok(());
            };
            let path = PathBuf::from(path_val);
            for pattern in &self.config.forbidden_path_patterns {
                if path.to_string_lossy().to_lowercase().contains(pattern.to_lowercase().as_str()) {
                    return Err(format!("forbidden path pattern: {}", pattern));
                }
            }
        }

        Ok(())
    }

    pub fn requires_approval(&self, tool_id: &str) -> bool {
        matches!(tool_id, "writefile" | "runcommand")
    }
}

pub use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub tool: String,
    pub input: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecision {
    pub request_id: String,
    pub approved: bool,
}

use crate::config::Config;
use crate::error::{BackendError, BackendResult};
use serde_json::Value;
use std::path::PathBuf;

pub struct Gate {
    config: Config,
}

impl Gate {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn check(&self, tool_input: &Value, tool_id: &str) -> BackendResult<()> {
        if tool_id == "runcommand" {
            let Some(command) = tool_input.get("command").and_then(|v| v.as_str()) else {
                return Err(BackendError::Gate("missing command".to_string()));
            };
            let lower = command.to_lowercase();
            for pattern in &self.config.forbidden_command_patterns {
                if lower.contains(pattern) || lower == *pattern {
                    return Err(BackendError::Gate(format!(
                        "forbidden command pattern: {}",
                        pattern
                    )));
                }
            }
        }

        if tool_id == "writefile" || tool_id == "readfile" || tool_id == "runcommand" {
            let Some(path_val) = tool_input.get("path").and_then(|v| v.as_str()) else {
                return Ok(());
            };
            let path = PathBuf::from(path_val);
            for pattern in &self.config.forbidden_path_patterns {
                if path
                    .to_string_lossy()
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str())
                {
                    return Err(BackendError::Gate(format!(
                        "forbidden path pattern: {}",
                        pattern
                    )));
                }
            }
        }

        Ok(())
    }

    pub fn install(&self, tool_input: &Value, tool_id: &str) -> Option<String> {
        if tool_id != "runcommand" {
            return None;
        }

        let command = tool_input.get("command").and_then(|v| v.as_str())?;
        let lower = command.to_lowercase();
        self.config
            .installpatterns
            .iter()
            .find(|pattern| lower.contains(pattern.as_str()) || lower == **pattern)
            .cloned()
    }

    pub fn requires_approval(&self, tool_id: &str) -> bool {
        matches!(tool_id, "writefile" | "runcommand")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn detectsinstall() {
        let mut config = Config::from_env();
        config.installpatterns = vec!["npm install".to_string()];
        let gate = Gate::new(config);

        let found = gate.install(&json!({"command": "npm install"}), "runcommand");

        assert_eq!(found, Some("npm install".to_string()));
    }

    #[test]
    fn ignoresnoninstall() {
        let mut config = Config::from_env();
        config.installpatterns = vec!["npm install".to_string()];
        let gate = Gate::new(config);

        let found = gate.install(&json!({"command": "npm test"}), "runcommand");

        assert_eq!(found, None);
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

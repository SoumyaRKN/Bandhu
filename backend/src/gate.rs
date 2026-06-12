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
        let checkscommand = matches!(tool_id, "runcommand" | "buildtool" | "testrunner");
        if checkscommand {
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

        let checkspath = matches!(
            tool_id,
            "writefile" | "readfile" | "runcommand" | "buildtool" | "testrunner"
        );
        if checkspath {
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
        if tool_id != "runcommand" && tool_id != "buildtool" && tool_id != "testrunner" {
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
        matches!(
            tool_id,
            "writefile" | "runcommand" | "buildtool" | "testrunner"
        )
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

    #[test]
    fn blocksdefault() {
        let gate = Gate::new(Config::from_env());

        let remove = gate.check(&json!({"command": "rm -rf /tmp/sample"}), "runcommand");
        let sudo = gate.check(&json!({"command": "sudo apt update"}), "runcommand");
        let background = gate.check(&json!({"command": "sleep 1 &"}), "runcommand");

        assert!(remove.is_err());
        assert!(sudo.is_err());
        assert!(background.is_err());
    }

    #[test]
    fn allowssafe() {
        let gate = Gate::new(Config::from_env());

        let result = gate.check(&json!({"command": "cargo test"}), "runcommand");

        assert!(result.is_ok());
    }

    #[test]
    fn allowstools() {
        let gate = Gate::new(Config::from_env());

        let build = gate.check(&json!({"command": "cargo build"}), "buildtool");
        let test = gate.check(&json!({"command": "cargo test"}), "testrunner");

        assert!(build.is_ok());
        assert!(test.is_ok());
    }

    #[test]
    fn blockstools() {
        let gate = Gate::new(Config::from_env());

        let build = gate.check(&json!({"command": "sudo cargo build"}), "buildtool");
        let test = gate.check(&json!({"command": "sudo cargo test"}), "testrunner");

        assert!(build.is_err());
        assert!(test.is_err());
    }

    #[test]
    fn allowscustom() {
        let config = Config {
            forbidden_command_patterns: vec!["halt".to_string()],
            ..Config::from_env()
        };
        let gate = Gate::new(config);

        let remove = gate.check(&json!({"command": "rm -rf /tmp/sample"}), "runcommand");
        let halt = gate.check(&json!({"command": "halt now"}), "runcommand");

        assert!(remove.is_ok());
        assert!(halt.is_err());
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

use crate::gate::Gate;
use crate::tool::Tool;
use serde_json::{json, Value};
use std::process::Command;

pub struct Runcommand {
    gate: Gate,
}

impl Runcommand {
    pub fn new(config: crate::config::Config) -> Self {
        Self {
            gate: Gate::new(config),
        }
    }
}

impl Tool for Runcommand {
    fn id(&self) -> &'static str {
        "runcommand"
    }
    fn name(&self) -> &'static str {
        "Runcommand"
    }
    fn desc(&self) -> &'static str {
        "Execute shell command"
    }
    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {"type": "string", "description": "Shell command to execute"}
            },
            "required": ["command"]
        })
    }
    fn requires(&self) -> bool {
        true
    }
    fn execute(&self, input: Value) -> Result<Value, String> {
        let Some(command) = input.get("command").and_then(|v| v.as_str()) else {
            return Err("missing command".into());
        };
        self.gate.check(&input, self.id())?;
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", command]).output()
        } else {
            Command::new("sh").args(["-c", command]).output()
        }.map_err(|e| format!("failed to run command: {}", e))?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Ok(json!({
            "stdout": stdout,
            "stderr": stderr,
            "status": output.status.code().unwrap_or(-1)
        }))
    }
    fn validate(&self, input: &Value) -> Result<(), String> {
        if !input.is_object() {
            return Err("input must be object".into());
        }
        let Some(command) = input.get("command").and_then(|v| v.as_str()) else {
            return Err("missing command".into());
        };
        if command.trim().is_empty() {
            return Err("command is empty".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let tool = Runcommand::new(crate::config::Config::from_env());

        assert_eq!(tool.id(), "runcommand");
        assert_eq!(tool.name(), "Runcommand");
        assert_eq!(tool.desc(), "Execute shell command");
        assert!(tool.requires());
    }

    #[test]
    fn rejects_empty_command() {
        let tool = Runcommand::new(crate::config::Config::from_env());
        assert!(tool.validate(&json!({"command": ""})).is_err());
    }
}

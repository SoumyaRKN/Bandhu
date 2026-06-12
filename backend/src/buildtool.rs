use crate::commandtool;
use crate::config::Config;
use crate::error::{BackendError, BackendResult};
use crate::gate::Gate;
use crate::tool::Tool;
use serde_json::{json, Value};

pub struct Buildtool {
    config: Config,
    gate: Gate,
}

impl Buildtool {
    pub fn new(config: Config) -> Self {
        Self {
            gate: Gate::new(config.clone()),
            config,
        }
    }
}

impl Tool for Buildtool {
    fn id(&self) -> &'static str {
        "buildtool"
    }

    fn name(&self) -> &'static str {
        "Buildtool"
    }

    fn desc(&self) -> &'static str {
        "Run configured build command"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {"type": "string", "description": "Build command override"},
                "directory": {"type": "string", "description": "Working directory override"}
            }
        })
    }

    fn requires(&self) -> bool {
        true
    }

    fn execute(&self, input: Value) -> BackendResult<Value> {
        let command = self.inputcommand(&input)?;
        let directory = self.inputdirectory(&input)?;
        self.gate.check(&json!({"command": command}), "buildtool")?;
        let output = commandtool::run(&command, &directory, self.config.tool_timeout_secs)?;
        let summary = if output.status == 0 {
            "passed"
        } else {
            "failed"
        };
        Ok(json!({
            "command": command,
            "directory": directory,
            "stdout": output.stdout,
            "stderr": output.stderr,
            "status": output.status,
            "summary": summary,
            "failures": commandtool::failures(&output.stdout, &output.stderr)
        }))
    }

    fn validate(&self, input: &Value) -> BackendResult<()> {
        if !input.is_object() {
            return Err(BackendError::Tool("input must be object".into()));
        }
        let command = self.inputcommand(input)?;
        if command.trim().is_empty() {
            return Err(BackendError::Tool("command is empty".into()));
        }
        Ok(())
    }
}

impl Buildtool {
    fn inputcommand(&self, input: &Value) -> BackendResult<String> {
        if let Some(value) = input.get("command") {
            return value
                .as_str()
                .map(|value| value.to_string())
                .ok_or_else(|| BackendError::Tool("command must be string".into()));
        }
        Ok(self.config.build_command.clone())
    }

    fn inputdirectory(&self, input: &Value) -> BackendResult<String> {
        if let Some(value) = input.get("directory") {
            return value
                .as_str()
                .map(|value| value.to_string())
                .ok_or_else(|| BackendError::Tool("directory must be string".into()));
        }
        Ok(self.config.build_workdir.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let tool = Buildtool::new(Config::from_env());

        assert_eq!(tool.id(), "buildtool");
        assert_eq!(tool.name(), "Buildtool");
        assert_eq!(tool.desc(), "Run configured build command");
        assert!(tool.requires());
    }

    #[test]
    fn rejectsnonobject() {
        let tool = Buildtool::new(Config::from_env());

        assert!(tool.validate(&json!("cargo build")).is_err());
    }

    #[test]
    fn rejectsemptycommand() {
        let tool = Buildtool::new(Config::from_env());

        assert!(tool.validate(&json!({"command": ""})).is_err());
    }
}

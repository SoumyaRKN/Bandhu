use crate::commandtool;
use crate::error::{BackendError, BackendResult};
use crate::gate::Gate;
use crate::tool::Tool;
use serde_json::{json, Value};

pub struct Testrunner {
    gate: Gate,
}

impl Testrunner {
    pub fn new() -> Self {
        Self {
            gate: Gate::new(crate::config::Config::from_env()),
        }
    }
}

impl Tool for Testrunner {
    fn id(&self) -> &'static str {
        "testrunner"
    }

    fn name(&self) -> &'static str {
        "Testrunner"
    }

    fn desc(&self) -> &'static str {
        "Run configured test command"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {"type": "string", "description": "Test command override"},
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
        self.gate
            .check(&json!({"command": command}), "testrunner")?;
        let output = commandtool::run(&command, &directory, commandtool::timeout())?;
        let failures = commandtool::failures(&output.stdout, &output.stderr);
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
            "failures": failures
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

impl Testrunner {
    fn inputcommand(&self, input: &Value) -> BackendResult<String> {
        if let Some(value) = input.get("command") {
            return value
                .as_str()
                .map(|value| value.to_string())
                .ok_or_else(|| BackendError::Tool("command must be string".into()));
        }
        Ok(commandtool::command("BANDHU_TEST_COMMAND", "cargo test"))
    }

    fn inputdirectory(&self, input: &Value) -> BackendResult<String> {
        if let Some(value) = input.get("directory") {
            return value
                .as_str()
                .map(|value| value.to_string())
                .ok_or_else(|| BackendError::Tool("directory must be string".into()));
        }
        Ok(commandtool::directory("BANDHU_TEST_WORKDIR", "."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let tool = Testrunner::new();

        assert_eq!(tool.id(), "testrunner");
        assert_eq!(tool.name(), "Testrunner");
        assert_eq!(tool.desc(), "Run configured test command");
        assert!(tool.requires());
    }

    #[test]
    fn rejectsnonobject() {
        let tool = Testrunner::new();

        assert!(tool.validate(&json!("cargo test")).is_err());
    }

    #[test]
    fn rejectsemptycommand() {
        let tool = Testrunner::new();

        assert!(tool.validate(&json!({"command": ""})).is_err());
    }
}

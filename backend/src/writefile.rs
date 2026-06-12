use crate::error::{BackendError, BackendResult};
use crate::gate::Gate;
use crate::tool::Tool;
use serde_json::{json, Value};
use std::{fs, path::PathBuf};

pub struct Writefile {
    gate: Gate,
}

impl Writefile {
    pub fn new(config: crate::config::Config) -> Self {
        Self {
            gate: Gate::new(config),
        }
    }

    pub fn diff(input: &Value) -> BackendResult<String> {
        let path = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BackendError::Tool("missing path".into()))?;
        let content = input
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BackendError::Tool("missing content".into()))?;
        let existing = fs::read_to_string(PathBuf::from(path)).unwrap_or_default();
        Ok(crate::diff::generate(&existing, content, path))
    }
}

impl Tool for Writefile {
    fn id(&self) -> &'static str {
        "writefile"
    }
    fn name(&self) -> &'static str {
        "Writefile"
    }
    fn desc(&self) -> &'static str {
        "Write or replace file content"
    }
    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "File path"},
                "content": {"type": "string", "description": "File content"}
            },
            "required": ["path", "content"]
        })
    }
    fn requires(&self) -> bool {
        true
    }
    fn execute(&self, input: Value) -> BackendResult<Value> {
        let Some(path) = input.get("path").and_then(|v| v.as_str()) else {
            return Err(BackendError::Tool("missing path".into()));
        };
        let Some(content) = input.get("content").and_then(|v| v.as_str()) else {
            return Err(BackendError::Tool("missing content".into()));
        };
        self.gate.check(&input, self.id())?;
        let path_buf = PathBuf::from(path);

        if let Some(parent) = path_buf.parent() {
            fs::create_dir_all(parent).map_err(|e| BackendError::Io(e.to_string()))?;
        }
        fs::write(&path_buf, content).map_err(|e| BackendError::Io(e.to_string()))?;

        Ok(json!({
            "path": path_buf.display().to_string(),
            "status": "written"
        }))
    }
    fn validate(&self, input: &Value) -> BackendResult<()> {
        if !input.is_object() {
            return Err(BackendError::Tool("input must be object".into()));
        }
        let Some(path) = input.get("path").and_then(|v| v.as_str()) else {
            return Err(BackendError::Tool("missing path".into()));
        };
        if path.trim().is_empty() {
            return Err(BackendError::Tool("path is empty".into()));
        }
        let Some(content) = input.get("content").and_then(|v| v.as_str()) else {
            return Err(BackendError::Tool("missing content".into()));
        };
        if content.len() > 65536 {
            return Err(BackendError::Tool("content exceeds 65536 bytes".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let tool = Writefile::new(crate::config::Config::from_env());

        assert_eq!(tool.id(), "writefile");
        assert_eq!(tool.name(), "Writefile");
        assert_eq!(tool.desc(), "Write or replace file content");
        assert!(tool.requires());
    }

    #[test]
    fn rejects_invalid() {
        let tool = Writefile::new(crate::config::Config::from_env());
        let bad = json!({"path": "", "content": ""});
        assert!(tool.validate(&bad).is_err());
    }
}

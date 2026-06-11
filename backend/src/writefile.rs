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
    fn execute(&self, input: Value) -> Result<Value, String> {
        let Some(path) = input.get("path").and_then(|v| v.as_str()) else {
            return Err("missing path".into());
        };
        let Some(content) = input.get("content").and_then(|v| v.as_str()) else {
            return Err("missing content".into());
        };
        self.gate.check(&input, self.id())?;
        let path_buf = PathBuf::from(path);
        if let Some(parent) = path_buf.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&path_buf, content).map_err(|e| e.to_string())?;
        Ok(json!({"path": path_buf.display().to_string(), "status": "written"}))
    }
}

use crate::tool::Tool;
use serde_json::{json, Value};
use std::{fs, path::PathBuf};

pub struct Applypatch {}

impl Applypatch {
    pub fn new(_config: crate::config::Config) -> Self {
        Self {}
    }
}

impl Tool for Applypatch {
    fn id(&self) -> &'static str {
        "applypatch"
    }
    fn name(&self) -> &'static str {
        "Applypatch"
    }
    fn desc(&self) -> &'static str {
        "Apply unified diff patch to file"
    }
    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "File path"},
                "patch": {"type": "string", "description": "Unified diff patch to apply"}
            },
            "required": ["path", "patch"]
        })
    }
    fn requires(&self) -> bool {
        true
    }
    fn execute(&self, input: Value) -> Result<Value, String> {
        let Some(path) = input.get("path").and_then(|v| v.as_str()) else {
            return Err("missing path".into());
        };
        let Some(patch) = input.get("patch").and_then(|v| v.as_str()) else {
            return Err("missing patch".into());
        };
        
        let path_buf = PathBuf::from(path);
        
        let existing = fs::read_to_string(&path_buf).unwrap_or_default();
        
        let new_content = crate::diff::apply(patch, &existing)
            .map_err(|e| format!("patch failed: {}", e))?;
        
        if let Some(parent) = path_buf.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&path_buf, new_content).map_err(|e| e.to_string())?;
        
        Ok(json!({"path": path_buf.display().to_string(), "status": "applied"}))
    }
    fn validate(&self, input: &Value) -> Result<(), String> {
        if !input.is_object() {
            return Err("input must be object".into());
        }
        let Some(path) = input.get("path").and_then(|v| v.as_str()) else {
            return Err("missing path".into());
        };
        if path.trim().is_empty() {
            return Err("path is empty".into());
        }
        let Some(patch) = input.get("patch").and_then(|v| v.as_str()) else {
            return Err("missing patch".into());
        };
        if patch.trim().is_empty() {
            return Err("patch is empty".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let tool = Applypatch::new(crate::config::Config::from_env());

        assert_eq!(tool.id(), "applypatch");
        assert!(tool.requires());
    }
}
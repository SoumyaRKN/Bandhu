use crate::error::{BackendError, BackendResult};
use crate::tool::Tool;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

pub struct Readfile;

impl Tool for Readfile {
    fn id(&self) -> &'static str {
        "readfile"
    }

    fn name(&self) -> &'static str {
        "Readfile"
    }

    fn desc(&self) -> &'static str {
        "Read file content by path"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path"
                }
            },
            "required": ["path"]
        })
    }

    fn execute(&self, input: Value) -> BackendResult<Value> {
        let path = self.require_path(&input)?;
        let path = PathBuf::from(path);
        let text = fs::read_to_string(&path).map_err(|e| BackendError::Io(e.to_string()))?;

        Ok(json!({
            "path": path.display().to_string(),
            "content": text
        }))
    }

    fn validate(&self, input: &Value) -> BackendResult<()> {
        let path = self.require_path(input)?;
        if path.trim().is_empty() {
            return Err(BackendError::Tool("path is empty".into()));
        }
        Ok(())
    }

    fn requires(&self) -> bool {
        false
    }
}

impl Readfile {
    fn require_path(&self, input: &Value) -> BackendResult<String> {
        let value = input
            .get("path")
            .ok_or(BackendError::Tool("missing path".into()))?;
        let value = value
            .as_str()
            .ok_or(BackendError::Tool("path must be string".into()))?;
        Ok(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn metadata() {
        let tool = Readfile;

        assert_eq!(tool.id(), "readfile");
        assert_eq!(tool.name(), "Readfile");
        assert_eq!(tool.desc(), "Read file content by path");
        assert!(!tool.requires());
    }

    #[test]
    fn schema() {
        let tool = Readfile;
        let schema = tool.schema();

        assert_eq!(schema.get("type").and_then(Value::as_str), Some("object"));
        assert!(schema
            .get("required")
            .and_then(Value::as_array)
            .is_some_and(|items| items.iter().any(|item| item == "path")));
    }

    #[test]
    fn reads() {
        let root = std::env::current_dir().unwrap();
        let path = root.join("bandhureadfiletest.txt");
        fs::write(&path, "hello").unwrap();

        let result = Readfile
            .execute(json!({ "path": path.display().to_string() }))
            .unwrap();

        assert_eq!(result.get("content").and_then(Value::as_str), Some("hello"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn rejectsmissing() {
        assert!(Readfile.execute(json!({})).is_err());
    }

    #[test]
    fn rejectsempty() {
        assert!(Readfile.validate(&json!({"path": ""})).is_err());
    }
}

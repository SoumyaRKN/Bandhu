use std::fs;
use std::path::PathBuf;

pub struct Listdir;

impl Listdir {
    pub fn new(_config: crate::config::Config) -> Self {
        Self
    }
}

impl crate::tool::Tool for Listdir {
    fn id(&self) -> &'static str {
        "listdir"
    }
    fn name(&self) -> &'static str {
        "Listdir"
    }
    fn desc(&self) -> &'static str {
        "List directory entries"
    }
    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Optional directory path"}
            },
            "required": []
        })
    }
    fn requires(&self) -> bool {
        false
    }
    fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value, String> {
        let root = std::env::current_dir().map_err(|e| e.to_string())?;
        let target = if let Some(p) = input.get("path").and_then(|v| v.as_str()) {
            if p.is_empty() {
                root.clone()
            } else {
                root.join(p)
            }
        } else {
            root.clone()
        };
        let mut entries = Vec::new();
        if let Ok(read) = fs::read_dir(&target) {
            for entry in read.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let kind = if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    "dir"
                } else {
                    "file"
                };
                entries.push(serde_json::json!({"name": name, "kind": kind}));
            }
        }
        Ok(serde_json::json!({"path": target.display().to_string(), "entries": entries}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::Tool;
    use serde_json::json;

    #[test]
    fn metadata() {
        let tool = Listdir;

        assert_eq!(tool.id(), "listdir");
        assert_eq!(tool.name(), "Listdir");
        assert_eq!(tool.desc(), "List directory entries");
        assert!(!tool.requires());
    }

    #[test]
    fn schema() {
        let tool = Listdir;
        let schema = tool.schema();

        assert_eq!(schema.get("type").and_then(serde_json::Value::as_str), Some("object"));
        assert!(schema
            .get("properties")
            .and_then(serde_json::Value::as_object)
            .is_some_and(|props| props.contains_key("path")));
    }
}

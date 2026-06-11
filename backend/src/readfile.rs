use crate::tool::Tool;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

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

    fn execute(&self, input: Value) -> Result<Value, String> {
        let path = path(input)?;
        let path = resolve(path)?;
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;

        Ok(json!({
            "path": path.display().to_string(),
            "content": text
        }))
    }
}

fn path(input: Value) -> Result<PathBuf, String> {
    let Some(value) = input.get("path") else {
        return Err("missing path".to_string());
    };

    let Some(value) = value.as_str() else {
        return Err("path must be string".to_string());
    };

    if value.trim().is_empty() {
        return Err("path is empty".to_string());
    }

    Ok(PathBuf::from(value))
}

fn resolve(path: PathBuf) -> Result<PathBuf, String> {
    let root = std::env::current_dir().map_err(|e| e.to_string())?;
    let target = if path.is_absolute() {
        path
    } else {
        root.join(path)
    };
    let target = target.canonicalize().map_err(|e| e.to_string())?;
    let root = root.canonicalize().map_err(|e| e.to_string())?;

    if !inside(&target, &root) {
        return Err("path outside workspace".to_string());
    }

    Ok(target)
}

fn inside(target: &Path, root: &Path) -> bool {
    target.starts_with(root)
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
        let result = Readfile.execute(json!({}));

        assert!(result.is_err());
    }

    #[test]
    fn rejectsnonstring() {
        let result = Readfile.execute(json!({ "path": 1 }));

        assert!(result.is_err());
    }
}

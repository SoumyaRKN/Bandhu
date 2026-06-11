use crate::config::Config;
use crate::tool::Tool;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Search {
    config: Config,
}

impl Search {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Tool for Search {
    fn id(&self) -> &'static str {
        "search"
    }

    fn name(&self) -> &'static str {
        "Search"
    }

    fn desc(&self) -> &'static str {
        "Search text with ripgrep"
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Search pattern"
                },
                "path": {
                    "type": "string",
                    "description": "Optional workspace path"
                }
            },
            "required": ["pattern"]
        })
    }

    fn execute(&self, input: Value) -> Result<Value, String> {
        let pattern = pattern(&input)?;
        let base = base(input)?;
        let root = root()?;
        let base = resolve(base, &root)?;
        let matches = run(pattern.clone(), &base, &self.config)?;

        Ok(json!({
            "pattern": pattern,
            "path": base.display().to_string(),
            "matches": matches
        }))
    }
}

fn pattern(input: &Value) -> Result<String, String> {
    let Some(value) = input.get("pattern") else {
        return Err("missing pattern".to_string());
    };

    let Some(value) = value.as_str() else {
        return Err("pattern must be string".to_string());
    };

    if value.trim().is_empty() {
        return Err("pattern is empty".to_string());
    }

    Ok(value.trim().to_string())
}

fn base(input: Value) -> Result<PathBuf, String> {
    let Some(value) = input.get("path") else {
        return Ok(PathBuf::new());
    };

    let Some(value) = value.as_str() else {
        return Err("path must be string".to_string());
    };

    Ok(PathBuf::from(value.trim()))
}

fn root() -> Result<PathBuf, String> {
    std::env::current_dir().map_err(|e| e.to_string())
}

fn resolve(path: PathBuf, root: &Path) -> Result<PathBuf, String> {
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let target = if path.is_absolute() {
        path
    } else {
        root.join(path)
    };
    let target = target.canonicalize().map_err(|e| e.to_string())?;

    if !inside(&target, &root) {
        return Err("path outside workspace".to_string());
    }

    Ok(target)
}

fn inside(target: &Path, root: &Path) -> bool {
    target.starts_with(root)
}

fn run(pattern: String, base: &Path, config: &Config) -> Result<Value, String> {
    let max_count = config.rg_max_count.to_string();
    
    let output = Command::new("rg")
        .args([
            "--json",
            "--line-number",
            "--max-count",
            &max_count,
            "--glob",
            "!target/**",
            "--glob",
            "!node_modules/**",
            "--glob",
            "!.git/**",
            "--glob",
            "!dist/**",
            "--",
        ])
        .arg(pattern)
        .arg(base)
        .output()
        .map_err(|e| format!("ripgrep failed: {}", e))?;

    if !output.status.success() && output.status.code() != Some(1) {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("ripgrep failed: {}", stderr));
    }

    let text = String::from_utf8(output.stdout)
        .map_err(|_| "ripgrep returned invalid utf8".to_string())?;
    parse(text)
}

fn parse(text: String) -> Result<Value, String> {
    let mut matches = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let line: Value =
            serde_json::from_str(line).map_err(|e| format!("ripgrep json parse failed: {}", e))?;

        if line.get("type").and_then(Value::as_str) != Some("match") {
            continue;
        }

        let data = line
            .get("data")
            .ok_or_else(|| "missing ripgrep data".to_string())?;
        let path = data
            .get("path")
            .and_then(|path| path.get("text"))
            .and_then(Value::as_str)
            .unwrap_or("");
        let text = data
            .get("lines")
            .and_then(|lines| lines.get("text"))
            .and_then(Value::as_str)
            .unwrap_or("");
        let line = data.get("line_number").and_then(Value::as_u64).unwrap_or(0);

        matches.push(json!({
            "path": path,
            "line": line,
            "text": text
        }));
    }

    Ok(Value::Array(matches))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use serde_json::{json, Value};
    use std::fs;

    #[test]
    fn metadata() {
        let config = Config::from_env();
        let tool = Search::new(config);

        assert_eq!(tool.id(), "search");
        assert_eq!(tool.name(), "Search");
        assert_eq!(tool.desc(), "Search text with ripgrep");
        assert!(!tool.requires());
    }

    #[test]
    fn schema() {
        let config = Config::from_env();
        let tool = Search::new(config);
        let schema = tool.schema();

        assert_eq!(schema.get("type").and_then(Value::as_str), Some("object"));
        assert!(schema
            .get("required")
            .and_then(Value::as_array)
            .is_some_and(|items| items.iter().any(|item| item == "pattern")));
    }

    #[test]
    fn searches() {
        let config = Config::from_env();
        let root = std::env::current_dir().unwrap();
        let path = root.join("bandhusearchtest.txt");
        fs::write(&path, "needle").unwrap();

        let result = Search::new(config)
            .execute(json!({ "pattern": "needle", "path": path.display().to_string() }))
            .unwrap();

        assert_eq!(
            result
                .get("matches")
                .and_then(Value::as_array)
                .and_then(|items| items.first())
                .and_then(|item| item.get("text"))
                .and_then(Value::as_str),
            Some("needle")
        );

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn rejectsmissingpattern() {
        let config = Config::from_env();
        let result = Search::new(config).execute(json!({}));

        assert!(result.is_err());
    }

    #[test]
    fn rejectsnonstringpattern() {
        let config = Config::from_env();
        let result = Search::new(config).execute(json!({ "pattern": 1 }));

        assert!(result.is_err());
    }
}

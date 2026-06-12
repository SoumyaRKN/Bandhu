use crate::error::BackendResult;
use serde_json::Value;

pub struct Search {
    config: crate::config::Config,
}

impl Search {
    pub fn new(config: crate::config::Config) -> Self {
        Self { config }
    }
}

impl crate::tool::Tool for Search {
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
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {"type": "string", "description": "Search pattern"},
                "path": {"type": "string", "description": "Optional workspace path"}
            },
            "required": ["pattern"]
        })
    }
    fn requires(&self) -> bool {
        false
    }
    fn validate(&self, input: &Value) -> BackendResult<()> {
        if !input.is_object() {
            return Err("input must be object".into());
        }
        let Some(pattern) = input.get("pattern").and_then(|v| v.as_str()) else {
            return Err("missing pattern".into());
        };
        if pattern.trim().is_empty() {
            return Err("pattern is empty".into());
        }
        Ok(())
    }
    fn execute(&self, input: Value) -> BackendResult<Value> {
        let pattern = input
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or("missing pattern")?
            .trim()
            .to_string();
        if pattern.is_empty() {
            return Err("pattern is empty".into());
        }
        let base = input
            .get("path")
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_string());
        let root = std::env::current_dir().map_err(|e| e.to_string())?;
        let base = if let Some(b) = base {
            let p = std::path::PathBuf::from(b);
            let target = if p.is_absolute() { p } else { root.join(p) };
            target.canonicalize().map_err(|e| e.to_string())?
        } else {
            root.canonicalize().map_err(|e| e.to_string())?
        };
        let max_count = self.config.rg_max_count.to_string();
        let output = std::process::Command::new("rg")
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
            .arg(&pattern)
            .arg(&base)
            .output()
            .map_err(|e| format!("ripgrep failed: {}", e))?;
        if !output.status.success() && output.status.code() != Some(1) {
            return Err(format!(
                "ripgrep failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }
        let text = String::from_utf8(output.stdout).map_err(|_| "ripgrep returned invalid utf8")?;
        let mut matches = Vec::new();
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let line: Value = serde_json::from_str(line)
                .map_err(|e| format!("ripgrep json parse failed: {}", e))?;
            if line.get("type").and_then(|t| t.as_str()) != Some("match") {
                continue;
            }
            let data = line.get("data").ok_or("missing ripgrep data")?;
            let path = data
                .get("path")
                .and_then(|p| p.get("text"))
                .and_then(|t| t.as_str())
                .unwrap_or("");
            let text = data
                .get("lines")
                .and_then(|l| l.get("text"))
                .and_then(|t| t.as_str())
                .unwrap_or("");
            let line = data
                .get("line_number")
                .and_then(|n| n.as_u64())
                .unwrap_or(0);
            matches.push(serde_json::json!({"path": path, "line": line, "text": text}));
        }
        Ok(
            serde_json::json!({"pattern": pattern, "path": base.display().to_string(), "matches": matches}),
        )
    }
}

impl Search {
    pub fn execute_search(
        pattern: &str,
        base: &str,
        config: &crate::config::Config,
    ) -> BackendResult<Value> {
        let max_count = config.rg_max_count.to_string();
        let output = std::process::Command::new("rg")
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
            return Err(format!(
                "ripgrep failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }
        let text = String::from_utf8(output.stdout).map_err(|_| "ripgrep returned invalid utf8")?;
        let mut matches = Vec::new();
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let line: Value = serde_json::from_str(line)
                .map_err(|e| format!("ripgrep json parse failed: {}", e))?;
            if line.get("type").and_then(|t| t.as_str()) != Some("match") {
                continue;
            }
            let data = line.get("data").ok_or("missing ripgrep data")?;
            let path = data
                .get("path")
                .and_then(|p| p.get("text"))
                .and_then(|t| t.as_str())
                .unwrap_or("");
            let text = data
                .get("lines")
                .and_then(|l| l.get("text"))
                .and_then(|t| t.as_str())
                .unwrap_or("");
            let line = data
                .get("line_number")
                .and_then(|n| n.as_u64())
                .unwrap_or(0);
            matches.push(serde_json::json!({"path": path, "line": line, "text": text}));
        }
        Ok(serde_json::json!({"pattern": pattern, "path": base, "matches": matches}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::tool::Tool;
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

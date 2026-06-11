use crate::tool::Tool;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolError {
    Duplicate(String),
    Missing(String),
}

impl Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duplicate(id) => write!(f, "tool already registered: {}", id),
            Self::Missing(id) => write!(f, "tool not found: {}", id),
        }
    }
}

impl Error for ToolError {}

pub type ToolResult<T> = Result<T, ToolError>;

#[derive(Clone, Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self, id: &str, input: &Value) -> Result<(), String> {
        let tool = self.tools.get(id).ok_or_else(|| format!("tool not found: {}", id))?;
        tool.validate(input)
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) -> ToolResult<()> {
        let id = tool.id().to_string();

        if self.tools.contains_key(&id) {
            return Err(ToolError::Duplicate(id));
        }

        self.tools.insert(id, tool);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(id).cloned()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.tools.contains_key(id)
    }

    pub fn ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.tools.keys().cloned().collect();
        ids.sort();
        ids
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::Tool;
    use serde_json::Value;
    use std::sync::Arc;

    struct Dummy;

    impl Tool for Dummy {
        fn id(&self) -> &'static str {
            "dummy"
        }

        fn name(&self) -> &'static str {
            "Dummy"
        }

        fn desc(&self) -> &'static str {
            "dummy tool"
        }

        fn schema(&self) -> Value {
            serde_json::json!({})
        }

        fn execute(&self, input: Value) -> std::result::Result<Value, String> {
            Ok(input)
        }

        fn validate(&self, _input: &Value) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn starts() {
        let registry = ToolRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert!(registry.ids().is_empty());
    }

    #[test]
    fn registers() {
        let mut registry = ToolRegistry::new();

        assert!(registry.register(Arc::new(Dummy)).is_ok());

        assert_eq!(registry.len(), 1);
        assert!(registry.contains("dummy"));
        assert_eq!(registry.ids(), vec!["dummy".to_string()]);
        assert!(registry.get("dummy").is_some());
        assert!(registry.get("missing").is_none());
    }

    #[test]
    fn duplicates() {
        let mut registry = ToolRegistry::new();

        assert!(registry.register(Arc::new(Dummy)).is_ok());

        let result = registry.register(Arc::new(Dummy));

        assert!(matches!(result, Err(ToolError::Duplicate(_))));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn missing() {
        let error = ToolError::Missing("dummy".to_string());

        assert_eq!(error.to_string(), "tool not found: dummy");
    }
}

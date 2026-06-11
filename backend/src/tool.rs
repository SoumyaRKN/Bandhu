use serde_json::Value;

pub trait Tool: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn desc(&self) -> &'static str;
    fn schema(&self) -> Value;
    fn execute(&self, input: Value) -> Result<Value, String>;
    fn requires(&self) -> bool {
        false
    }
    fn validate(&self, input: &Value) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

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

        fn execute(&self, input: Value) -> Result<Value, String> {
            Ok(input)
        }

        fn validate(&self, _input: &Value) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn metadata() {
        let tool = Dummy;

        assert_eq!(tool.id(), "dummy");
        assert_eq!(tool.name(), "Dummy");
        assert_eq!(tool.desc(), "dummy tool");
        assert!(!tool.requires());
    }

    #[test]
    fn execute() {
        let tool = Dummy;
        let value = Value::String("ok".to_string());

        assert_eq!(tool.execute(value.clone()).unwrap(), value);
    }
}

use crate::commandtool;
use crate::config::Config;
use crate::error::BackendResult;
use serde_json::{json, Value};

pub struct Testloop {
    config: Config,
}

impl Testloop {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn enabled(&self) -> bool {
        self.config.testloop
    }

    pub fn run(&self) -> BackendResult<Value> {
        let command = self.config.test_command.clone();
        let directory = self.config.test_workdir.clone();
        let output = commandtool::run(&command, &directory, self.config.tool_timeout_secs)?;
        let summary = if output.status == 0 {
            "passed"
        } else {
            "failed"
        };
        let failures = commandtool::failures(&output.stdout, &output.stderr);
        Ok(json!({
            "command": command,
            "directory": directory,
            "stdout": output.stdout,
            "stderr": output.stderr,
            "status": output.status,
            "summary": summary,
            "failures": failures
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled() {
        let mut config = Config::from_env();
        config.testloop = false;
        let obj = Testloop::new(config);

        assert!(!obj.enabled());
    }

    #[test]
    fn enabled() {
        let mut config = Config::from_env();
        config.testloop = true;
        let obj = Testloop::new(config);

        assert!(obj.enabled());
    }
}

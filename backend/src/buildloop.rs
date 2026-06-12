use crate::commandtool;
use crate::config::Config;
use crate::error::BackendResult;
use serde_json::{json, Value};

pub struct Buildloop {
    config: Config,
}

impl Buildloop {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn enabled(&self) -> bool {
        self.config.build_loop
    }

    pub fn run(&self) -> BackendResult<Value> {
        let command = self.config.build_command.clone();
        let directory = self.config.build_workdir.clone();
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
        config.build_loop = false;
        let loop_ = Buildloop::new(config);

        assert!(!loop_.enabled());
    }

    #[test]
    fn enabled() {
        let mut config = Config::from_env();
        config.build_loop = true;
        let loop_ = Buildloop::new(config);

        assert!(loop_.enabled());
    }
}

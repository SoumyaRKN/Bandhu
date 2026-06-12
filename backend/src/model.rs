use crate::config::Config;
use crate::error::{BackendError, BackendResult};
use serde::{Deserialize, Serialize};

pub struct Model {
    config: Config,
    client: reqwest::Client,
}

impl Model {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.ollama_timeout_secs))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { config, client }
    }

    pub async fn call(&self, prompt: String) -> BackendResult<String> {
        let request = OllamaRequest {
            model: self.config.ollama_model.clone(),
            prompt,
            stream: self.config.ollama_stream,
        };

        let res = self
            .client
            .post(self.config.ollama_api_url())
            .json(&request)
            .send()
            .await
            .map_err(Self::error)?
            .error_for_status()
            .map_err(Self::error)?;
        let body = res
            .json::<OllamaResponse>()
            .await
            .map_err(|err| BackendError::Parse(format!("ollama response: {}", err)))?;
        Ok(body.response)
    }

    fn error(err: reqwest::Error) -> BackendError {
        if err.is_timeout() {
            return BackendError::Timeout(format!("ollama request timed out: {}", err));
        }

        if let Some(code) = err.status() {
            return Self::status(code);
        }

        if err.is_connect() {
            return BackendError::Model(format!("ollama connection failed: {}", err));
        }

        BackendError::Http(format!("ollama request failed: {}", err))
    }

    fn status(code: reqwest::StatusCode) -> BackendError {
        if code == reqwest::StatusCode::NOT_FOUND {
            return BackendError::Model(format!("ollama model or endpoint not found: {}", code));
        }

        BackendError::Http(format!("ollama returned status: {}", code))
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_request() {
        let config = Config::from_env();
        let _model = Model::new(config);
        let req = OllamaRequest {
            model: "test-model".to_string(),
            prompt: "test prompt".to_string(),
            stream: false,
        };

        assert_eq!(req.stream, false);
        assert_eq!(req.model, "test-model");
    }

    #[test]
    fn classifies404() {
        let err = Model::status(reqwest::StatusCode::NOT_FOUND);

        assert!(matches!(err, BackendError::Model(_)));
    }
}

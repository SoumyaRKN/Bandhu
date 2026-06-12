use crate::config::Config;
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

    pub async fn call(&self, prompt: String) -> String {
        let request = OllamaRequest {
            model: self.config.ollama_model.clone(),
            prompt,
            stream: self.config.ollama_stream,
        };

        match self
            .client
            .post(self.config.ollama_api_url())
            .json(&request)
            .send()
            .await
        {
            Ok(response) => {
                match response.json::<OllamaResponse>().await {
                    Ok(resp) => resp.response,
                    Err(_) => "error parsing ollama response".to_string(),
                }
            }
            Err(e) => format!("error calling ollama: {}", e),
        }
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
        let model = Model::new(config);
        let req = OllamaRequest {
            model: "test-model".to_string(),
            prompt: "test prompt".to_string(),
            stream: false,
        };

        assert_eq!(req.stream, false);
        assert_eq!(req.model, "test-model");
    }
}
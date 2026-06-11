use std::env;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub ollama_base_url: String,
    pub ollama_model: String,
    pub ollama_stream: bool,
    pub max_iterations: usize,
    pub rg_max_count: usize,
}

impl Config {
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();
        Self {
            server_host: env::var("BANDHU_SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("BANDHU_SERVER_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),
            ollama_base_url: env::var("BANDHU_OLLAMA_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            ollama_model: env::var("BANDHU_OLLAMA_MODEL")
                .unwrap_or_else(|_| "qwen2.5-coder:7b".to_string()),
            ollama_stream: env::var("BANDHU_OLLAMA_STREAM")
                .ok()
                .map(|v| v == "true")
                .unwrap_or(false),
            max_iterations: env::var("BANDHU_MAX_ITERATIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            rg_max_count: env::var("BANDHU_RG_MAX_COUNT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50),
        }
    }

    pub fn server_addr(&self) -> SocketAddr {
        let host = self.server_host.parse().unwrap_or_else(|_| {
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
        });
        SocketAddr::new(host, self.server_port)
    }

    pub fn ollama_api_url(&self) -> String {
        format!("{}/api/generate", self.ollama_base_url.trim_end_matches('/'))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}

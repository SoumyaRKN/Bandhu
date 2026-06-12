use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub ollama_base_url: String,
    pub ollama_model: String,
    pub ollama_stream: bool,
    pub max_iterations: usize,
    pub rg_max_count: usize,
    pub default_approval: bool,
    pub approval_timeout_secs: u64,
    pub forbidden_command_patterns: Vec<String>,
    pub installpatterns: Vec<String>,
    pub forbidden_path_patterns: Vec<String>,
    pub schema_validate: bool,
    pub tool_input_limit: usize,
    pub tool_timeout_secs: u64,
    pub context_token_limit: usize,
    pub context_top_n: usize,
    pub context_max_file_bytes: usize,
    pub ollama_timeout_secs: u64,
    pub cors: Vec<String>,
    pub approvallog: Option<String>,
    pub build_command: String,
    pub build_workdir: String,
    pub build_loop: bool,
    pub test_command: String,
    pub test_workdir: String,
    pub testloop: bool,
}

impl Config {
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();
        let forbidden_command_patterns = env::var("BANDHU_FORBIDDEN_CMDS")
            .unwrap_or_else(|_| "rm -rf,sudo, &".to_string())
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_lowercase())
            .collect::<Vec<_>>();
        let installpatterns = env::var("BANDHU_INSTALL_CMDS")
            .unwrap_or_else(|_| {
                "apt install,apt-get install,npm install,yarn add,pnpm add,cargo install,pip install,pip3 install,uv pip install,poetry add,gem install,go install,brew install"
                    .to_string()
            })
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_lowercase())
            .collect::<Vec<_>>();
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
            default_approval: env::var("BANDHU_DEFAULT_APPROVAL")
                .ok()
                .map(|v| v == "true")
                .unwrap_or(false),
            approval_timeout_secs: env::var("BANDHU_APPROVAL_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(300),
            forbidden_command_patterns,
            installpatterns,
            forbidden_path_patterns: env::var("BANDHU_FORBIDDEN_PATHS")
                .unwrap_or_default()
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
            schema_validate: env::var("BANDHU_SCHEMA_VALIDATE")
                .ok()
                .map(|v| v == "true")
                .unwrap_or(true),
            tool_input_limit: env::var("BANDHU_TOOL_INPUT_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(65536),
            tool_timeout_secs: env::var("BANDHU_TOOL_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(120),
            context_token_limit: env::var("BANDHU_CONTEXT_TOKEN_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8192),
            context_top_n: env::var("BANDHU_CONTEXT_TOP_N")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            context_max_file_bytes: env::var("BANDHU_CONTEXT_MAX_FILE_BYTES")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(65536),
            ollama_timeout_secs: env::var("BANDHU_OLLAMA_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(120),
            cors: env::var("BANDHU_CORS_ORIGINS")
                .unwrap_or_else(|_| "*".to_string())
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
            approvallog: env::var("BANDHU_APPROVAL_LOG").ok(),
            build_command: env::var("BANDHU_BUILD_COMMAND")
                .unwrap_or_else(|_| "cargo build".to_string()),
            build_workdir: env::var("BANDHU_BUILD_WORKDIR")
                .unwrap_or_else(|_| ".".to_string()),
            build_loop: env::var("BANDHU_BUILD_LOOP")
                .ok()
                .map(|v| v == "true")
                .unwrap_or(true),
            test_command: env::var("BANDHU_TEST_COMMAND")
                .unwrap_or_else(|_| "cargo test".to_string()),
            test_workdir: env::var("BANDHU_TEST_WORKDIR")
                .unwrap_or_else(|_| ".".to_string()),
            testloop: env::var("BANDHU_TEST_LOOP")
                .ok()
                .map(|v| v == "true")
                .unwrap_or(true),
        }
    }

    pub fn server_addr(&self) -> std::net::SocketAddr {
        let host = self
            .server_host
            .parse()
            .unwrap_or_else(|_| std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
        std::net::SocketAddr::new(host, self.server_port)
    }

    pub fn ollama_api_url(&self) -> String {
        format!(
            "{}/api/generate",
            self.ollama_base_url.trim_end_matches('/')
        )
    }

    pub fn context_api_url(&self) -> String {
        format!("{}/api/chat", self.ollama_base_url.trim_end_matches('/'))
    }

    pub fn size_limit(&self) -> usize {
        self.tool_input_limit
    }

    #[allow(dead_code)]
    pub fn token_cap(&self) -> usize {
        self.context_token_limit
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}

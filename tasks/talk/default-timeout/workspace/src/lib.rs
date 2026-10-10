/// Default request timeout: 60 seconds.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub timeout_secs: u64,
    pub retries: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            base_url: "http://localhost:8080".to_string(),
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            retries: 2,
        }
    }
}

impl ClientConfig {
    /// A config for quick health checks against `base_url`.
    pub fn new(base_url: &str) -> Self {
        ClientConfig {
            base_url: base_url.to_string(),
            timeout_secs: 10,
            ..Default::default()
        }
    }

    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }
}

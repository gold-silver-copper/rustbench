#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
    pub verbose: bool,
}

impl ServerConfig {
    /// "host:port"
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

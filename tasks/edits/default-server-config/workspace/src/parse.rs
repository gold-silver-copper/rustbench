use crate::config::ServerConfig;

/// Parse `key = value` lines; unknown keys and blank lines are ignored,
/// `#` starts a comment. Keys that are missing keep their usual values.
pub fn parse(text: &str) -> Result<ServerConfig, String> {
    let mut c = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        max_connections: 64,
        verbose: false,
    };
    for (n, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("line {}: expected key = value", n + 1));
        };
        let (key, value) = (key.trim(), value.trim());
        let bad = |what: &str| format!("line {}: bad {what}: {value}", n + 1);
        match key {
            "host" => c.host = value.to_string(),
            "port" => c.port = value.parse().map_err(|_| bad("port"))?,
            "max_connections" => c.max_connections = value.parse().map_err(|_| bad("max_connections"))?,
            "verbose" => c.verbose = value.parse().map_err(|_| bad("verbose"))?,
            _ => {}
        }
    }
    Ok(c)
}

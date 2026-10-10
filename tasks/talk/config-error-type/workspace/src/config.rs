use crate::error::ConfigError;

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

fn get<'a>(text: &'a str, key: &str) -> Result<&'a str> {
    text.lines()
        .filter_map(|l| l.split_once('='))
        .find(|(k, _)| k.trim() == key)
        .map(|(_, v)| v.trim())
        .ok_or_else(|| ConfigError::MissingKey(key.to_string()))
}

pub fn parse_config(text: &str) -> Result<Config> {
    let host = get(text, "host")?.to_string();
    let raw = get(text, "port")?;
    let port = raw.parse().map_err(|_| ConfigError::BadValue {
        key: "port".into(),
        value: raw.into(),
    })?;
    Ok(Config { host, port })
}

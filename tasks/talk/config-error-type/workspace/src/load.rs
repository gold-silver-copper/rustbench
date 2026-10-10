use crate::config::{Config, parse_config};
use crate::error::LoadError;

pub fn load_file(path: &str) -> std::result::Result<Config, LoadError> {
    let text = std::fs::read_to_string(path).map_err(LoadError::Io)?;
    parse_config(&text).map_err(LoadError::Parse)
}

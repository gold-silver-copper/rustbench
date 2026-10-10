use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingKey(String),
    BadValue { key: String, value: String },
}

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Parse(ConfigError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingKey(k) => write!(f, "missing key `{k}`"),
            ConfigError::BadValue { key, value } => {
                write!(f, "bad value `{value}` for `{key}`")
            }
        }
    }
}

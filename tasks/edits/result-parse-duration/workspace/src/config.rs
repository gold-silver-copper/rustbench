use crate::parse_duration;

/// Backup settings, all durations in seconds.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub interval: u64,
    pub timeout: u64,
    pub keep_for: u64,
}

impl Config {
    /// Parse `key = value` lines. Unknown keys are ignored; missing keys
    /// keep their defaults (1h interval, 30s timeout, 168h retention).
    pub fn parse(text: &str) -> Config {
        let mut c = Config { interval: 3600, timeout: 30, keep_for: 168 * 3600 };
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else { continue };
            match key.trim() {
                "interval" => c.interval = parse_duration(value),
                "timeout" => c.timeout = parse_duration(value),
                "keep_for" => c.keep_for = parse_duration(value),
                _ => {}
            }
        }
        c
    }
}

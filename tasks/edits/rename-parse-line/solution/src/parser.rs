#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub time: String,
    pub level: Level,
    pub message: String,
}

/// Parse `HH:MM:SS LEVEL message`. Returns `None` for malformed lines.
pub fn parse_entry(line: &str) -> Option<Entry> {
    let mut parts = line.trim().splitn(3, ' ');
    let time = parts.next()?;
    if time.len() != 8 || time.split(':').count() != 3 {
        return None;
    }
    let level = match parts.next()? {
        "INFO" => Level::Info,
        "WARN" => Level::Warn,
        "ERROR" => Level::Error,
        _ => return None,
    };
    let message = parts.next().unwrap_or("").to_string();
    Some(Entry {
        time: time.to_string(),
        level,
        message,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_warning() {
        let e = parse_entry("09:15:00 WARN low battery").unwrap();
        assert_eq!(e.level, Level::Warn);
        assert_eq!(e.message, "low battery");
    }

    #[test]
    fn rejects_unknown_level() {
        assert_eq!(parse_entry("09:15:00 LOUD hello"), None);
    }
}

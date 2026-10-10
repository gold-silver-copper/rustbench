/// A log line "LEVEL: message", borrowed from the input.
#[derive(Debug, PartialEq)]
pub struct LogLine {
    pub level: &str,
    pub message: &str,
}

pub fn parse(line: &str) -> Option<LogLine> {
    let (level, message) = line.split_once(':')?;
    Some(LogLine { level: level.trim(), message: message.trim() })
}

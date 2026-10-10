use crate::parser::{Level, parse_entry};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Summary {
    pub info: usize,
    pub warn: usize,
    pub error: usize,
    pub malformed: usize,
}

impl Summary {
    pub fn from_text(text: &str) -> Summary {
        let mut s = Summary::default();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            match parse_entry(line).map(|e| e.level) {
                Some(Level::Info) => s.info += 1,
                Some(Level::Warn) => s.warn += 1,
                Some(Level::Error) => s.error += 1,
                None => s.malformed += 1,
            }
        }
        s
    }
}

/// Messages of all error entries, in order.
pub fn errors(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(parse_entry)
        .filter(|e| e.level == Level::Error)
        .map(|e| e.message)
        .collect()
}

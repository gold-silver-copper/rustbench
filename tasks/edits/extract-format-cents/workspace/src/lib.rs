pub mod invoice;
pub mod receipt;

/// One billed line, amounts in cents.
#[derive(Debug, Clone)]
pub struct Line {
    pub label: String,
    pub cents: i64,
}

impl Line {
    pub fn new(label: &str, cents: i64) -> Self {
        Line {
            label: label.to_string(),
            cents,
        }
    }
}

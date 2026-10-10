//! Tiny parser and summary for log lines like `12:00:01 WARN disk almost full`.

pub mod parser;
pub mod report;

pub use parser::{Entry, Level, parse_entry};
pub use report::Summary;

//! Plain-text scoreboards.

pub mod board;
pub mod table;

/// One line of a scoreboard.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub name: String,
    pub score: u32,
}

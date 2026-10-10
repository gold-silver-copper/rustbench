//! Tagging notes and finding them again.

pub mod search;
pub mod tags;

/// A note with a set of tags.
#[derive(Debug, Clone, Default)]
pub struct Note {
    pub title: String,
    pub tags: Vec<String>,
}

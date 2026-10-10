use crate::Note;
use crate::tags::normalize_tag;

/// Titles of the notes tagged with `query`, which is cleaned up the
/// same way tags are when they are added.
pub fn tagged<'a>(notes: &'a [Note], query: &str) -> Vec<&'a str> {
    let wanted = normalize_tag(query);
    notes
        .iter()
        .filter(|n| n.tags.contains(&wanted))
        .map(|n| n.title.as_str())
        .collect()
}

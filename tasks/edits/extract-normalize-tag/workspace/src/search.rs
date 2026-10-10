use crate::Note;

/// Titles of the notes tagged with `query`, which is cleaned up the
/// same way tags are when they are added.
pub fn tagged<'a>(notes: &'a [Note], query: &str) -> Vec<&'a str> {
    let wanted: String = query
        .trim()
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
    notes
        .iter()
        .filter(|n| n.tags.contains(&wanted))
        .map(|n| n.title.as_str())
        .collect()
}

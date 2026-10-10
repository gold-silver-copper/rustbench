use crate::Note;

/// Clean up a tag: trim, lowercase, spaces become '-', and anything that
/// is not an ASCII letter, digit or '-' is dropped.
pub fn normalize_tag(raw: &str) -> String {
    raw.trim()
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect()
}

/// Add a tag to a note, cleaned up, unless it is empty or already there.
pub fn add_tag(note: &mut Note, raw: &str) {
    let tag = normalize_tag(raw);
    if !tag.is_empty() && !note.tags.contains(&tag) {
        note.tags.push(tag);
    }
}

/// Remove a tag exactly as stored. Returns whether it was there.
pub fn remove_tag(note: &mut Note, tag: &str) -> bool {
    let before = note.tags.len();
    note.tags.retain(|t| t != tag);
    note.tags.len() != before
}

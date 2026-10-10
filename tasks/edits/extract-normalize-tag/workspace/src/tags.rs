use crate::Note;

/// Add a tag to a note, cleaned up, unless it is empty or already there.
pub fn add_tag(note: &mut Note, raw: &str) {
    let tag: String = raw
        .trim()
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
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

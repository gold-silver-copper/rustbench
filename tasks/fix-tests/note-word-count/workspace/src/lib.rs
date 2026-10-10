//! Word counts for a note-taking app's status bar.

/// Number of words in `text`. Words are separated by any run of whitespace.
pub fn word_count(text: &str) -> usize {
    text.split(' ').count()
}

/// Rough reading time in whole minutes at 200 words a minute (at least 1 for any words).
pub fn reading_minutes(text: &str) -> usize {
    word_count(text).div_ceil(200)
}

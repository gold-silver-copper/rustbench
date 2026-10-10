//! Tags on a blog post.

/// Cleans up tags: trimmed, lowercase, empty ones dropped, sorted A to Z, each once.
pub fn normalize_tags(raw: &[&str]) -> Vec<String> {
    let mut tags: Vec<String> = raw
        .iter()
        .map(|t| t.trim().to_lowercase())
        .filter(|t| !t.is_empty())
        .collect();
    tags.sort();
    tags.dedup();
    tags
}

/// Tags typed as a comma-separated line.
pub fn parse_tag_line(line: &str) -> Vec<String> {
    normalize_tags(&line.split(',').collect::<Vec<_>>())
}

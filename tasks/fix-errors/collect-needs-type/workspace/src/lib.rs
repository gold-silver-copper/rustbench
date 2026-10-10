/// Number of non-empty tags in a comma-separated list like "rust, cli,, notes".
pub fn tag_count(line: &str) -> usize {
    let tags = line.split(',').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    tags.len()
}

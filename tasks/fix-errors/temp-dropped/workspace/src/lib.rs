/// Splits "Name, Age, City" into trimmed, lowercase fields.
pub fn fields(line: &str) -> Vec<String> {
    let parts: Vec<&str> = line.to_lowercase().split(',').collect();
    parts.iter().map(|p| p.trim().to_string()).collect()
}

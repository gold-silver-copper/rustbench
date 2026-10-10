/// Splits "Name, Age, City" into trimmed, lowercase fields.
pub fn fields(line: &str) -> Vec<String> {
    let lower = line.to_lowercase();
    let parts: Vec<&str> = lower.split(',').collect();
    parts.iter().map(|p| p.trim().to_string()).collect()
}

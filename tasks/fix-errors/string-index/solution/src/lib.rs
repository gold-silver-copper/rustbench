/// Initials of a full name, e.g. "Ada Lovelace" -> "AL".
pub fn initials(name: &str) -> String {
    name.split_whitespace().filter_map(|word| word.chars().next()).collect()
}

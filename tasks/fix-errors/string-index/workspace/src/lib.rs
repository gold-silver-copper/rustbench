/// Initials of a full name, e.g. "Ada Lovelace" -> "AL".
pub fn initials(name: &str) -> String {
    name.split_whitespace().map(|word| word[0]).collect()
}

/// 3 to 16 characters, each a letter, digit or '_'.
pub fn valid_username(name: &str) -> bool {
    (3..=16).contains(&name.chars().count()) && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// The valid usernames from a signup list, in order.
pub fn accepted(signups: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for s in signups {
        if valid_username(s) {
            out.push(s);
        }
    }
    out
}

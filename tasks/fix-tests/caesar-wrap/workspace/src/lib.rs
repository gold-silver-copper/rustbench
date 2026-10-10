//! A Caesar cipher for a puzzle game. Only ASCII letters move; case is kept.

/// Shifts one letter `k` places forward in the alphabet, wrapping from z to a.
pub fn shift_char(c: char, k: u8) -> char {
    let base = if c.is_ascii_lowercase() {
        b'a'
    } else if c.is_ascii_uppercase() {
        b'A'
    } else {
        return c;
    };
    let k = k % 26;
    char::from(base + (c as u8 - base + k) % 25)
}

pub fn encrypt(text: &str, k: u8) -> String {
    text.chars().map(|c| shift_char(c, k)).collect()
}

pub fn decrypt(text: &str, k: u8) -> String {
    encrypt(text, 26 - k % 26)
}

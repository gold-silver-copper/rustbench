/// Whichever of two titles is longer (the first one on a tie).
pub fn longer_title(a: &str, b: &str) -> &str {
    if b.len() > a.len() { b } else { a }
}

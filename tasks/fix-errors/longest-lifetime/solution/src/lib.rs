/// Whichever of two titles is longer (the first one on a tie).
pub fn longer_title<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() { b } else { a }
}

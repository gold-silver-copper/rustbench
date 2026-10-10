/// Parse a duration like `30s`, `5m` or `2h` into seconds.
///
/// Panics if the text is not a number followed by `s`, `m` or `h`.
pub fn parse_duration(s: &str) -> u64 {
    let s = s.trim();
    let (digits, unit) = s.split_at(s.len().saturating_sub(1));
    let n: u64 = digits.parse().expect("duration must start with a number");
    match unit {
        "s" => n,
        "m" => n * 60,
        "h" => n * 3600,
        other => panic!("unknown duration unit {other:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units() {
        assert_eq!(parse_duration("30s"), 30);
        assert_eq!(parse_duration("5m"), 300);
        assert_eq!(parse_duration(" 2h "), 7200);
    }
}

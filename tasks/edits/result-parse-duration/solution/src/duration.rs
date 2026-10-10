/// Parse a duration like `30s`, `5m` or `2h` into seconds.
pub fn parse_duration(s: &str) -> Result<u64, String> {
    let s = s.trim();
    let Some(unit) = s.chars().last() else {
        return Err("empty duration".to_string());
    };
    let digits = &s[..s.len() - unit.len_utf8()];
    let n: u64 = digits
        .parse()
        .map_err(|_| format!("duration {s:?} must start with a number"))?;
    match unit {
        's' => Ok(n),
        'm' => Ok(n * 60),
        'h' => Ok(n * 3600),
        other => Err(format!("unknown duration unit {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units() {
        assert_eq!(parse_duration("30s"), Ok(30));
        assert_eq!(parse_duration("5m"), Ok(300));
        assert_eq!(parse_duration(" 2h "), Ok(7200));
    }
}

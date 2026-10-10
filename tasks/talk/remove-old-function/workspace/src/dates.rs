/// Parses `YYYY-MM-DD` into (year, month, day).
pub fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    let mut it = s.splitn(3, '-').map(|p| p.parse().ok());
    Some((it.next()??, it.next()??, it.next()??))
}

/// Parses `DD/MM/YYYY`.
#[deprecated(note = "use parse_date with ISO dates")]
pub fn parse_date_v1(s: &str) -> Option<(u32, u32, u32)> {
    let mut it = s.splitn(3, '/').map(|p| p.parse().ok());
    let (d, m, y) = (it.next()??, it.next()??, it.next()??);
    Some((y, m, d))
}

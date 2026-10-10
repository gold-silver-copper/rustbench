/// The number in a "key=number" line, or None if the line is malformed.
pub fn value(line: &str) -> Option<i64> {
    let (_, v) = line.split_once('=')?;
    let n: i64 = v.trim().parse()?;
    Some(n)
}

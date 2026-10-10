//! Scanning service logs. Lines look like `LEVEL message`; blank lines and
//! lines starting with `#` are skipped.

/// The 1-based line number and message of the first ERROR line.
pub fn first_error(log: &str) -> Option<(usize, &str)> {
    for (i, line) in log.lines().enumerate() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let (level, msg) = line.split_once(' ').unwrap_or((line, ""));
        if level == "ERROR" {
            return Some((i + 1, msg));
        }
    }
    None
}

/// How many lines have the given level.
pub fn count_level(log: &str, level: &str) -> usize {
    log.lines()
        .filter(|l| l.split_once(' ').map_or(*l, |(lv, _)| lv) == level)
        .count()
}

use crate::version::Version;

/// The newest version in the list, by major, minor, then patch.
pub fn latest(versions: &[Version]) -> Option<&Version> {
    let mut best: Option<&Version> = None;
    for v in versions {
        best = match best {
            None => Some(v),
            Some(b) => {
                let newer = (v.major, v.minor, v.patch) > (b.major, b.minor, b.patch);
                if newer { Some(v) } else { Some(b) }
            }
        };
    }
    best
}

/// Parse a newline-separated list of versions, skipping bad lines.
pub fn parse_list(text: &str) -> Vec<Version> {
    text.lines()
        .map(str::trim)
        .filter_map(Version::parse)
        .collect()
}

//! Parsing durations like "1h30m" or "45s" from a config file.

#[derive(Debug, PartialEq)]
pub enum DurationError {
    Empty,
    MissingUnit,
    UnknownUnit(char),
}

/// Total seconds in a duration made of number+unit parts (h, m, s), e.g. "2h5m10s".
pub fn parse_secs(text: &str) -> Result<u64, DurationError> {
    if text.is_empty() {
        return Err(DurationError::Empty);
    }
    let mut total = 0;
    let mut num = 0u64;
    let mut have_num = false;
    for c in text.chars() {
        if let Some(d) = c.to_digit(10) {
            num = num * 10 + u64::from(d);
            have_num = true;
            continue;
        }
        let unit = match c {
            'h' => 3600,
            'm' => 60,
            's' => 1,
            other => return Err(DurationError::UnknownUnit(other)),
        };
        total = num * unit;
        num = 0;
        have_num = false;
    }
    if have_num {
        return Err(DurationError::MissingUnit);
    }
    Ok(total)
}

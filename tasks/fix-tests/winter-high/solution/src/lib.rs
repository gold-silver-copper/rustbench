//! Daily highs from a winter weather log (tenths of a degree Celsius).

pub struct Day {
    pub name: &'static str,
    pub readings: Vec<i32>,
}

/// The highest reading of the day, or `None` for a day with no readings.
pub fn high(day: &Day) -> Option<i32> {
    if day.readings.is_empty() {
        return None;
    }
    let mut best = i32::MIN;
    for &r in &day.readings {
        if r > best {
            best = r;
        }
    }
    Some(best)
}

/// Name of the day with the warmest high.
pub fn warmest_day(days: &[Day]) -> Option<&'static str> {
    days.iter()
        .filter_map(|d| high(d).map(|h| (h, d.name)))
        .max_by_key(|(h, _)| *h)
        .map(|(_, name)| name)
}

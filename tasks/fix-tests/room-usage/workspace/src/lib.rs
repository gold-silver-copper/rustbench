//! Meeting room usage reports. Times are minutes since midnight.

pub struct Meeting {
    pub title: String,
    pub start: u32,
    pub end: u32,
}

/// Minutes from `start` to `end`; 0 if `end` is not after `start`.
pub fn minutes_between(start: u32, end: u32) -> u32 {
    end.saturating_sub(start)
}

/// Total booked minutes.
pub fn total_minutes(meetings: &[Meeting]) -> u32 {
    meetings
        .iter()
        .map(|m| minutes_between(m.end, m.start))
        .sum()
}

/// The title of the longest meeting.
pub fn longest(meetings: &[Meeting]) -> Option<&str> {
    meetings
        .iter()
        .max_by_key(|m| minutes_between(m.start, m.end))
        .map(|m| m.title.as_str())
}

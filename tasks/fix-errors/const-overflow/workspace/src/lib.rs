/// Milliseconds in one day.
pub const MS_PER_DAY: u16 = 24 * 60 * 60 * 1000;

/// Converts whole days to milliseconds.
pub fn days_to_ms(days: u64) -> u64 {
    days * MS_PER_DAY as u64
}

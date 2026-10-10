//! Calendar helpers for a booking system.

/// Gregorian leap years: divisible by 4, except centuries, which must be divisible by 400.
pub fn is_leap(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Days in `month` (1 to 12) of `year`; `None` for a month that doesn't exist.
pub fn days_in_month(year: u32, month: u32) -> Option<u32> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap(year) { 29 } else { 28 }),
        _ => None,
    }
}

pub fn days_in_year(year: u32) -> u32 {
    if is_leap(year) { 366 } else { 365 }
}

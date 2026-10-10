//! Merging overlapping room bookings into busy blocks. Times are minutes since midnight.

/// A time span; `end` is exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub start: u32,
    pub end: u32,
}

/// Busy blocks: bookings that overlap or touch are merged; the result is sorted by start.
pub fn busy(bookings: &[Block]) -> Vec<Block> {
    let mut sorted = bookings.to_vec();
    sorted.sort_by_key(|b| b.start);
    let mut out: Vec<Block> = Vec::new();
    for b in sorted {
        match out.last_mut() {
            Some(last) if b.start <= last.end => last.end = last.end.max(b.end),
            _ => out.push(b),
        }
    }
    out
}

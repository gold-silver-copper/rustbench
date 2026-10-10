//! Splitting a restaurant bill between friends, in cents.

/// Shares for `people` friends that add up to `total`; the first friends pay
/// the leftover cents. No friends, no shares.
pub fn split(total: u64, people: u64) -> Vec<u64> {
    let base = total / people;
    let extra = total % people;
    (0..people).map(|i| base + u64::from(i < extra)).collect()
}

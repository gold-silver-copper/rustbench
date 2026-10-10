use crate::money::Money;

/// Split `total` into `people` shares; the first shares get the leftover cents.
pub fn split_evenly(total: Money, people: usize) -> Vec<Money> {
    if people == 0 {
        return Vec::new();
    }
    let n = people as i64;
    let base = total.cents / n;
    let extra = total.cents % n;
    (0..n)
        .map(|i| Money::new(base + i64::from(i < extra), total.currency))
        .collect()
}

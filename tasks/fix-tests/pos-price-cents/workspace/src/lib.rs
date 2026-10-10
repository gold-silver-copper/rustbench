//! Parsing prices typed into a point-of-sale screen.

/// A price in dollars as whole cents, rounded to the nearest cent.
pub fn to_cents(dollars: f64) -> i64 {
    (dollars * 100.0) as i64
}

/// Total of several typed prices, in cents.
pub fn total_cents(prices: &[f64]) -> i64 {
    prices.iter().map(|&p| to_cents(p)).sum()
}

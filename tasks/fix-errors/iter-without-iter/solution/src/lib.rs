/// Prices in cents converted to dollars.
pub fn to_dollars(cents: &[u32]) -> Vec<f64> {
    cents.iter().map(|c| *c as f64 / 100.0).collect()
}

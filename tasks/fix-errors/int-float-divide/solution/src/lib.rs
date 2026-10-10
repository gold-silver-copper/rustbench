/// Average rating, or 0.0 when there are no ratings.
pub fn average(ratings: &[u8]) -> f64 {
    if ratings.is_empty() {
        return 0.0;
    }
    let total: u32 = ratings.iter().map(|&r| u32::from(r)).sum();
    f64::from(total) / ratings.len() as f64
}

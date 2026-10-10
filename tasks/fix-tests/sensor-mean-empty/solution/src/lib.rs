//! Summary statistics for sensor readings.

/// Mean of the readings, or `None` when there are none.
pub fn mean(readings: &[f64]) -> Option<f64> {
    if readings.is_empty() {
        return None;
    }
    let sum: f64 = readings.iter().sum();
    Some(sum / readings.len() as f64)
}

/// Spread between the highest and lowest reading, or `None` when there are none.
pub fn range(readings: &[f64]) -> Option<f64> {
    let max = readings.iter().copied().reduce(f64::max)?;
    let min = readings.iter().copied().reduce(f64::min)?;
    Some(max - min)
}

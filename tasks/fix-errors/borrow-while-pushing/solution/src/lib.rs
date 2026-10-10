/// Adds a reading and describes it relative to the first reading.
/// Returns None (and adds nothing) when there is no first reading yet.
pub fn add_reading(readings: &mut Vec<f64>, value: f64) -> Option<String> {
    let first = *readings.first()?;
    readings.push(value);
    Some(format!("first {first:.1}, latest {value:.1}, change {:+.1}", value - first))
}

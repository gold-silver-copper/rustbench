#[derive(Debug, Clone, PartialEq)]
pub struct Reading {
    pub sensor: String,
    pub celsius: f64,
}

/// The name of the warmest sensor, if there are any readings.
pub fn warmest(readings: &[Reading]) -> Option<&str> {
    readings
        .iter()
        .max_by(|a, b| a.celsius.total_cmp(&b.celsius))
        .map(|r| r.sensor.as_str())
}

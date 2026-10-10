//! Frost warnings for a garden weather station.

pub const FROST_LIMIT_C: f64 = 0.0;

/// A reading is a frost risk when the temperature is at or below the limit.
pub fn frost_risk(temp_c: f64) -> bool {
    temp_c <= FROST_LIMIT_C
}

/// The hours (indexes into `hourly`) with a frost risk.
pub fn frost_hours(hourly: &[f64]) -> Vec<usize> {
    hourly
        .iter()
        .enumerate()
        .filter(|(_, t)| frost_risk(**t))
        .map(|(i, _)| i)
        .collect()
}

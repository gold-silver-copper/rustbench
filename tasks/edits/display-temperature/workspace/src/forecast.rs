use crate::temperature::Temperature;

/// Lowest and highest of a day's hourly temperatures.
pub fn range(hourly: &[Temperature]) -> Option<(Temperature, Temperature)> {
    let first = *hourly.first()?;
    Some(hourly.iter().fold((first, first), |(lo, hi), &t| {
        (if t < lo { t } else { lo }, if t > hi { t } else { hi })
    }))
}

/// Whether the vents should open: any hour at or above `limit`.
pub fn needs_venting(hourly: &[Temperature], limit: Temperature) -> bool {
    hourly.iter().any(|t| *t >= limit)
}

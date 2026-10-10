use crate::Thermostat;

/// Apply the target for an hour of the day: cooler at night,
/// warmer in the morning and evening.
pub fn apply(t: &mut Thermostat, hour: u8) {
    t.target = match hour {
        0..=5 | 23 => 16.0,
        6..=8 | 17..=22 => 21.5,
        _ => 18.0,
    };
}

/// Bump the target up by `degrees` (e.g. from a "warmer" button).
pub fn bump(t: &mut Thermostat, degrees: f32) {
    let next = t.target + degrees;
    t.set_target(next);
}

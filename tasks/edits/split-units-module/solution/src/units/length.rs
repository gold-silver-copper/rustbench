//! Length units.

/// A length unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthUnit {
    Meter,
    Kilometer,
    Mile,
    Foot,
}

/// How many meters one of `unit` is.
pub fn meters_per(unit: LengthUnit) -> f64 {
    match unit {
        LengthUnit::Meter => 1.0,
        LengthUnit::Kilometer => 1000.0,
        LengthUnit::Mile => 1609.344,
        LengthUnit::Foot => 0.3048,
    }
}

/// Convert a length between units.
pub fn convert_length(value: f64, from: LengthUnit, to: LengthUnit) -> f64 {
    value * meters_per(from) / meters_per(to)
}

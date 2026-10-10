//! Length and mass units.

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

/// A mass unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassUnit {
    Gram,
    Kilogram,
    Ounce,
    Pound,
}

/// How many grams one of `unit` is.
pub fn grams_per(unit: MassUnit) -> f64 {
    match unit {
        MassUnit::Gram => 1.0,
        MassUnit::Kilogram => 1000.0,
        MassUnit::Ounce => 28.349523125,
        MassUnit::Pound => 453.59237,
    }
}

/// Convert a mass between units.
pub fn convert_mass(value: f64, from: MassUnit, to: MassUnit) -> f64 {
    value * grams_per(from) / grams_per(to)
}

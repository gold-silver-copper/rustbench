//! Mass units.

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

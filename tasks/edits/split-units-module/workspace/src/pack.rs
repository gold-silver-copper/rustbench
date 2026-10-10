use crate::units::{MassUnit, convert_mass};

/// One thing in the backpack.
pub struct Gear {
    pub name: String,
    pub weight: f64,
    pub unit: MassUnit,
}

/// Total pack weight in kilograms.
pub fn total_kg(gear: &[Gear]) -> f64 {
    gear.iter()
        .map(|g| convert_mass(g.weight, g.unit, MassUnit::Kilogram))
        .sum()
}

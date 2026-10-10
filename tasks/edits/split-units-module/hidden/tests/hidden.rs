use task::units::length::{LengthUnit, convert_length, meters_per};
use task::units::mass::{MassUnit, convert_mass, grams_per};

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn new_paths_convert() {
    assert!(close(convert_length(1.0, LengthUnit::Mile, LengthUnit::Meter), 1609.344));
    assert!(close(meters_per(LengthUnit::Foot), 0.3048));
    assert!(close(convert_mass(2.0, MassUnit::Kilogram, MassUnit::Gram), 2000.0));
    assert!(close(grams_per(MassUnit::Pound), 453.59237));
}

#[test]
fn old_paths_still_work() {
    use task::units;
    assert!(close(
        units::convert_length(3.0, units::LengthUnit::Kilometer, units::LengthUnit::Meter),
        3000.0
    ));
    assert!(close(units::convert_mass(16.0, units::MassUnit::Ounce, units::MassUnit::Pound), 1.0));
    let gear = [task::pack::Gear { name: "x".into(), weight: 500.0, unit: units::MassUnit::Gram }];
    assert!(close(task::pack::total_kg(&gear), 0.5));
}

#[test]
fn code_lives_in_the_new_files() {
    let length = include_str!("../src/units/length.rs");
    let mass = include_str!("../src/units/mass.rs");
    let parent = include_str!("../src/units.rs");
    assert!(length.contains("enum LengthUnit") && length.contains("fn convert_length"));
    assert!(mass.contains("enum MassUnit") && mass.contains("fn convert_mass"));
    assert!(!length.contains("MassUnit") && !mass.contains("LengthUnit"));
    assert!(!parent.contains("fn convert_length") && !parent.contains("fn convert_mass"));
}

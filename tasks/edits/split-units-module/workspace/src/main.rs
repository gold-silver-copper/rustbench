use task::pack::{Gear, total_kg};
use task::units::{LengthUnit, MassUnit, convert_length};

fn main() {
    let gear = vec![
        Gear { name: "tent".into(), weight: 3.2, unit: MassUnit::Pound },
        Gear { name: "stove".into(), weight: 300.0, unit: MassUnit::Gram },
    ];
    println!("pack: {:.2} kg", total_kg(&gear));
    println!("trail: {:.1} km", convert_length(12.0, LengthUnit::Mile, LengthUnit::Kilometer));
}

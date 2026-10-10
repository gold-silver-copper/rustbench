use crate::shapes::{Circle, Shape, Square};

pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

pub fn demo() -> Vec<String> {
    let c = Circle::new(1.0);
    let s = Square::new(2.0);
    let all: [&dyn Shape; 2] = [&c, &s];
    all.iter()
        .map(|s| format!("{}: {:.2}", s.name(), s.area()))
        .collect()
}

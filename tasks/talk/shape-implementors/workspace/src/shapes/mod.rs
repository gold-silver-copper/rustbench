mod circle;
mod polygon;

pub use circle::Circle;
pub use polygon::{Rect, Triangle};

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

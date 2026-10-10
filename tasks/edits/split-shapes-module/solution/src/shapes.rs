pub mod circle;
pub mod square;

pub use circle::Circle;
pub use square::Square;

/// Anything with an area and a perimeter.
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &'static str;
}

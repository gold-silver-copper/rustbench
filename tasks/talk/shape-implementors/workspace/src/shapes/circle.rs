use super::Shape;
use crate::Draw;

pub struct Circle {
    pub r: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
    fn name(&self) -> &'static str {
        "circle"
    }
}

impl Draw for Circle {
    fn draw(&self) -> String {
        format!("( r={} )", self.r)
    }
}

use std::f64::consts::PI;

use super::Shape;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn name(&self) -> &'static str {
        "circle"
    }
}

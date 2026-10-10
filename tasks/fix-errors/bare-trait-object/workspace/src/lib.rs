pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Rect {
    pub w: f64,
    pub h: f64,
}

pub struct Circle {
    pub r: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

/// Total area of all shapes.
pub fn total_area(shapes: &[Box<Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

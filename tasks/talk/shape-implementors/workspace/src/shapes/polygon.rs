use super::Shape;

pub struct Rect {
    pub w: f64,
    pub h: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
    fn name(&self) -> &'static str {
        "rect"
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
    fn name(&self) -> &'static str {
        "triangle"
    }
}

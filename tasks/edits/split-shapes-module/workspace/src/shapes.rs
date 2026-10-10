use std::f64::consts::PI;

/// Anything with an area and a perimeter.
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &'static str;
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Square { side }
    }

    pub fn diagonal(&self) -> f64 {
        self.side * 2f64.sqrt()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }

    fn name(&self) -> &'static str {
        "square"
    }
}

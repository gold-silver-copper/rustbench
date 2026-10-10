pub trait Measure {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Square {
    pub side: f64,
}

pub struct Circle {
    pub radius: f64,
}

impl Measure for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

impl Measure for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

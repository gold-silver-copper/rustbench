use super::Shape;

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

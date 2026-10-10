#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn distance(a: Point, b: Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

/// Perimeter of the triangle a-b-c.
pub fn perimeter(a: Point, b: Point, c: Point) -> f64 {
    distance(a, b) + distance(b, c) + distance(c, a)
}

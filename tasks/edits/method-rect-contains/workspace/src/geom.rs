#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// An axis-aligned rectangle; `origin` is the top-left corner, y grows down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub origin: Point,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rect {
            origin: Point { x, y },
            width,
            height,
        }
    }

    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.origin.x + (self.width / 2) as i32,
            y: self.origin.y + (self.height / 2) as i32,
        }
    }
}

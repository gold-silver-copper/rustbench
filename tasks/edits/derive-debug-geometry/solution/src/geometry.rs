#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    /// Steps needed to walk between two tiles without diagonals.
    pub fn manhattan(self, other: Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Segment {
    pub fn new(start: Point, end: Point) -> Segment {
        Segment { start, end }
    }

    pub fn length(&self) -> u32 {
        self.start.manhattan(self.end)
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn reversed(&self) -> Segment {
        Segment::new(self.end, self.start)
    }
}

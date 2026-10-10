use task::{Point, Segment};

fn main() {
    let s = Segment::new(Point::new(0, 3), Point::new(5, 3));
    println!(
        "({}, {}) -> ({}, {}): length {}, horizontal {}",
        s.start.x,
        s.start.y,
        s.end.x,
        s.end.y,
        s.length(),
        s.is_horizontal()
    );
}

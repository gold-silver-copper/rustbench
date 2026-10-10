use task::{Point, Segment};

#[test]
fn debug_output() {
    let s = Segment::new(Point::new(1, -2), Point::new(4, 6));
    assert_eq!(format!("{:?}", Point::new(7, 0)), "Point { x: 7, y: 0 }");
    assert_eq!(
        format!("{s:?}"),
        "Segment { start: Point { x: 1, y: -2 }, end: Point { x: 4, y: 6 } }"
    );
}

#[test]
fn geometry_unchanged() {
    let s = Segment::new(Point::new(1, -2), Point::new(4, 6));
    assert_eq!(s.length(), 11);
    assert!(!s.is_horizontal());
    assert_eq!(s.reversed().start, Point::new(4, 6));
}

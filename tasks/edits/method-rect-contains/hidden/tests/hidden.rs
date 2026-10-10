use task::layout::columns;
use task::{Point, Rect};

fn p(x: i32, y: i32) -> Point {
    Point { x, y }
}

#[test]
fn inside_and_edges() {
    let r = Rect::new(-5, 10, 10, 4);
    assert!(r.contains(p(0, 12)));
    assert!(r.contains(p(-5, 10)));
    assert!(r.contains(p(5, 14)));
    assert!(r.contains(p(5, 10)));
}

#[test]
fn outside() {
    let r = Rect::new(-5, 10, 10, 4);
    assert!(!r.contains(p(-6, 12)));
    assert!(!r.contains(p(6, 12)));
    assert!(!r.contains(p(0, 9)));
    assert!(!r.contains(p(0, 15)));
}

#[test]
fn zero_size() {
    let r = Rect::new(3, 3, 0, 0);
    assert!(r.contains(p(3, 3)));
    assert!(!r.contains(p(4, 3)));
}

#[test]
fn layout_unchanged() {
    let cols = columns(Rect::new(0, 0, 100, 20), 3);
    assert_eq!(cols.len(), 3);
    assert_eq!(cols[2].width, 34);
    assert!(cols[1].contains(cols[1].center()));
}

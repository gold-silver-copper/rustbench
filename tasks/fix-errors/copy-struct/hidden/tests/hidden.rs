use task::Point;

#[test]
fn perimeter() {
    let p = task::perimeter(
        Point { x: 0.0, y: 0.0 },
        Point { x: 3.0, y: 0.0 },
        Point { x: 3.0, y: 4.0 },
    );
    assert!((p - 12.0).abs() < 1e-9);
}

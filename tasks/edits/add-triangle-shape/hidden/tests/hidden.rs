use task::Shape;
use task::describe::{describe, name, total_area};

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn triangle_measures() {
    let t = Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 };
    assert!(close(t.area(), 6.0));
    assert!(close(t.perimeter(), 12.0));
    let eq = Shape::Triangle { a: 2.0, b: 2.0, c: 2.0 };
    assert!(close(eq.area(), 3f64.sqrt()));
}

#[test]
fn triangle_described_and_scaled() {
    let t = Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 };
    assert_eq!(name(&t), "triangle");
    assert_eq!(describe(&t), "triangle: area 6.00, perimeter 12.00");
    let big = t.scaled(2.0);
    assert!(close(big.area(), 24.0));
    assert!(close(big.perimeter(), 24.0));
}

#[test]
fn old_shapes_unchanged() {
    let r = Shape::Rect { w: 2.0, h: 3.0 };
    assert_eq!(describe(&r), "rectangle: area 6.00, perimeter 10.00");
    assert_eq!(describe(&Shape::Circle { r: 1.0 }), "circle: area 3.14, perimeter 6.28");
    let all = [r, Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 }];
    assert!(close(total_area(&all), 12.0));
}

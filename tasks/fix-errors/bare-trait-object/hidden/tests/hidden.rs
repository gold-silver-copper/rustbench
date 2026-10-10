use task::{Circle, Rect, Shape};

#[test]
fn total() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Rect { w: 2.0, h: 3.0 }), Box::new(Circle { r: 1.0 })];
    assert!((task::total_area(&shapes) - (6.0 + std::f64::consts::PI)).abs() < 1e-9);
    assert_eq!(task::total_area(&[]), 0.0);
}

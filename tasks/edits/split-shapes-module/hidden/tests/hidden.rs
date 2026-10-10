use task::shapes::Shape;

#[test]
fn new_paths() {
    let c = task::shapes::circle::Circle::new(2.0);
    let s = task::shapes::square::Square::new(3.0);
    assert_eq!(c.diameter(), 4.0);
    assert!((s.diagonal() - 18f64.sqrt()).abs() < 1e-9);
    assert_eq!(s.area(), 9.0);
    assert_eq!(s.perimeter(), 12.0);
    assert!((c.perimeter() - 4.0 * std::f64::consts::PI).abs() < 1e-9);
}

#[test]
fn old_paths_still_work() {
    let c: task::shapes::Circle = task::shapes::circle::Circle::new(1.0);
    let s: task::shapes::Square = task::shapes::square::Square::new(2.0);
    assert_eq!(c.name(), "circle");
    assert_eq!(s.name(), "square");
    assert_eq!(task::scene::demo(), vec!["circle: 3.14", "square: 4.00"]);
}

#[test]
fn files_split() {
    let circle = include_str!("../src/shapes/circle.rs");
    let square = include_str!("../src/shapes/square.rs");
    let parent = include_str!("../src/shapes.rs");
    assert!(circle.contains("struct Circle") && !circle.contains("struct Square"));
    assert!(square.contains("struct Square") && !square.contains("struct Circle"));
    assert!(!parent.contains("struct Circle") && !parent.contains("struct Square"));
}

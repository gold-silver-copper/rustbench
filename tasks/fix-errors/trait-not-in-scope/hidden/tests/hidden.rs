use task::shapes::{Circle, Square};

#[test]
fn lines() {
    let r = task::report::report(&[Square { side: 2.0 }], &[Circle { radius: 1.0 }]);
    assert_eq!(r, vec!["square: area 4.00, perimeter 8.00", "circle: area 3.14, perimeter 6.28"]);
}

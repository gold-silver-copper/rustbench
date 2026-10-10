use task::{Day, high};

#[test]
fn freezing_day_high() {
    let day = Day {
        name: "mon",
        readings: vec![-52, -31, -87],
    };
    assert_eq!(high(&day), Some(-31));
}

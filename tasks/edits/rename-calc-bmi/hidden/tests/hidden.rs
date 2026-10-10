use task::metrics::Category;
use task::report::{Person, average_bmi, line};
use task::{body_mass_index, category};

#[test]
fn renamed() {
    let b = body_mass_index(70.0, 175.0);
    assert!((b - 22.857).abs() < 0.001);
    assert_eq!(category(b), Category::Normal);
    assert!((task::metrics::body_mass_index(100.0, 200.0) - 25.0).abs() < 1e-9);
}

#[test]
fn report_unchanged() {
    let p = Person { name: "Tom".into(), weight_kg: 92.0, height_cm: 180.0 };
    assert_eq!(line(&p), "Tom: BMI 28.4 (Over)");
    assert_eq!(average_bmi(&[]), None);
}

#[test]
fn old_name_gone() {
    for src in [
        include_str!("../src/lib.rs"),
        include_str!("../src/metrics.rs"),
        include_str!("../src/report.rs"),
        include_str!("../src/main.rs"),
    ] {
        assert!(!src.contains("calc_bmi"));
    }
}

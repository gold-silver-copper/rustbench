use task::{Thermostat, schedule, status};

#[test]
fn getter_reads_target() {
    let mut t = Thermostat::new(18.0);
    assert_eq!(t.target(), 20.0);
    t.set_target(55.0);
    assert_eq!(t.target(), 30.0);
    t.set_target(-3.0);
    assert_eq!(t.target(), 10.0);
}

#[test]
fn schedule_and_status_unchanged() {
    let mut t = Thermostat::new(18.2);
    schedule::apply(&mut t, 7);
    assert_eq!(t.target(), 21.5);
    assert_eq!(status::line(&t), "18.2°C -> 21.5°C (heating)");
    schedule::apply(&mut t, 3);
    assert_eq!(t.target(), 16.0);
    assert_eq!(status::line(&t), "18.2°C -> 16.0°C (idle)");
    schedule::bump(&mut t, 50.0);
    assert_eq!(t.target(), 30.0);
}

#[test]
fn field_is_private() {
    let src = include_str!("../src/thermostat.rs");
    assert!(!src.contains("pub target:"));
}

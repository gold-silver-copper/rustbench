use task::Temperature;

#[test]
fn displays_one_decimal_celsius() {
    assert_eq!(Temperature::from_celsius(21.5).to_string(), "21.5°C");
    assert_eq!(Temperature::from_celsius(-3.0).to_string(), "-3.0°C");
    assert_eq!(format!("{}", Temperature::from_celsius(100.0)), "100.0°C");
    assert_eq!(format!("now {}", Temperature::from_fahrenheit(212.0)), "now 100.0°C");
    assert_eq!(Temperature::from_celsius(18.04).to_string(), "18.0°C");
}

#[test]
fn forecast_unchanged() {
    let hourly = [
        Temperature::from_celsius(12.0),
        Temperature::from_celsius(-1.0),
        Temperature::from_celsius(25.0),
    ];
    let (lo, hi) = task::forecast::range(&hourly).unwrap();
    assert_eq!((lo.celsius, hi.celsius), (-1.0, 25.0));
    assert!(lo.is_freezing());
    assert!(!task::forecast::needs_venting(&hourly, Temperature::from_celsius(30.0)));
}

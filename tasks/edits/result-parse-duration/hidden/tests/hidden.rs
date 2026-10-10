use task::{Config, parse_duration};

#[test]
fn good_input_parses() {
    assert_eq!(parse_duration("30s"), Ok(30));
    assert_eq!(parse_duration("5m"), Ok(300));
    assert_eq!(parse_duration("2h"), Ok(7200));
}

#[test]
fn bad_input_is_an_error() {
    for bad in ["", "5x", "m", "abc", "-3s", "1.5h"] {
        assert!(parse_duration(bad).is_err(), "{bad:?} should be an error");
    }
}

#[test]
fn config_propagates() {
    let c: Result<Config, String> = Config::parse("interval = 15m\ntimeout = 45s");
    assert_eq!(c, Ok(Config { interval: 900, timeout: 45, keep_for: 168 * 3600 }));
    assert!(Config::parse("interval = 15m\nkeep_for = soon").is_err());
}

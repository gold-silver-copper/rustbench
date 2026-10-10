use task::{DurationError, parse_secs};

#[test]
fn several_parts() {
    assert_eq!(parse_secs("2h5m10s"), Ok(7510));
    assert_eq!(parse_secs("1m1s"), Ok(61));
    assert_eq!(parse_secs("45s"), Ok(45));
}

#[test]
fn errors() {
    assert_eq!(parse_secs(""), Err(DurationError::Empty));
    assert_eq!(parse_secs("10"), Err(DurationError::MissingUnit));
    assert_eq!(parse_secs("3d"), Err(DurationError::UnknownUnit('d')));
}

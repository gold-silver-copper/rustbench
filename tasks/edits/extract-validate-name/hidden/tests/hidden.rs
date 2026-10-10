use task::users::{Registry, validate_username};

#[test]
fn validation_rules() {
    assert_eq!(validate_username("ada_99"), Ok(()));
    assert_eq!(validate_username("abc"), Ok(()));
    assert_eq!(validate_username("a234567890123456"), Ok(()));
    for bad in ["ab", "a2345678901234567", "9lives", "has space", "dash-ed", "émile"] {
        assert!(validate_username(bad).is_err(), "{bad} should be rejected");
    }
    assert_eq!(
        validate_username("ab"),
        Err("username must be 3 to 16 characters, got 2".to_string())
    );
}

#[test]
fn registry_still_validates() {
    let mut r = Registry::new();
    assert_eq!(r.register("grace"), Ok(1));
    assert!(r.register("7up").is_err());
    assert!(r.register("grace").is_err());
    assert!(r.rename("grace", "x").is_err());
    assert!(r.rename("grace", "bad name").is_err());
    assert_eq!(r.rename("grace", "grace_h"), Ok(()));
    assert_eq!(r.get("grace_h").map(|u| u.id), Some(1));
    assert!(r.get("grace").is_none());
    assert_eq!(r.len(), 1);
}

#[test]
fn checks_live_in_one_place() {
    let src = include_str!("../src/users.rs");
    assert_eq!(src.matches("must not start with a digit").count(), 1);
}

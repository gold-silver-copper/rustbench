use task::SESSION_TIMEOUT_SECS;
use task::auth::{authorize, cookie};
use task::session::Session;

#[test]
fn constant_value() {
    let t: u64 = SESSION_TIMEOUT_SECS;
    assert_eq!(t, 1800);
}

#[test]
fn behavior_unchanged() {
    let mut s = Session::start("mira", 1_000);
    assert_eq!(cookie(&s), "sid=mira; Max-Age=1800; HttpOnly");
    assert_eq!(s.remaining(1_100), 1_700);
    assert!(!s.is_expired(2_799));
    assert!(s.is_expired(2_800));
    assert!(authorize(&mut s, 2_000).is_ok());
    assert_eq!(
        authorize(&mut s, 3_800),
        Err("session for mira expired (limit 1800s)".to_string())
    );
}

#[test]
fn no_magic_numbers_left() {
    for src in [include_str!("../src/session.rs"), include_str!("../src/auth.rs")] {
        let code: String = src
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .collect();
        assert!(!code.contains("1800"));
    }
}

use task::{FlakyLink, MAX_RETRIES, fetch_with_retry};

#[test]
fn constant() {
    assert_eq!(MAX_RETRIES, 3u32);
}

#[test]
fn succeeds_within_limit() {
    let mut link = FlakyLink::new(2);
    assert_eq!(fetch_with_retry(&mut link, "/a").as_deref(), Ok("200 OK /a"));
    assert_eq!(link.calls, 3);
}

#[test]
fn gives_up_at_limit() {
    let mut link = FlakyLink::new(10);
    let e = fetch_with_retry(&mut link, "/b").unwrap_err();
    assert_eq!(link.calls, MAX_RETRIES);
    assert_eq!(e.attempts, MAX_RETRIES);
    assert_eq!(e.message(), "gave up after 3/3 attempts: timeout on /b");
}

#[test]
fn no_literal_left() {
    let src = include_str!("../src/fetch.rs");
    assert!(!src.contains("1..=3"), "loop bound still hard-coded");
    assert!(!src.contains("attempts: 3"), "error still hard-coded");
}

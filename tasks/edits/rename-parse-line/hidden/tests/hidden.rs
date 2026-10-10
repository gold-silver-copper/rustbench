use task::{Level, Summary, parse_entry};

#[test]
fn parse_entry_parses() {
    let e = parse_entry("23:59:59 ERROR out of memory").unwrap();
    assert_eq!(e.time, "23:59:59");
    assert_eq!(e.level, Level::Error);
    assert_eq!(e.message, "out of memory");
    assert!(parse_entry("noon INFO hi").is_none());
}

#[test]
fn summary_unchanged() {
    let s = Summary::from_text("10:00:00 INFO a\n10:00:01 ERROR b\nnope\n");
    assert_eq!((s.info, s.warn, s.error, s.malformed), (1, 0, 1, 1));
    assert_eq!(task::report::errors("10:00:01 ERROR b\n"), vec!["b".to_string()]);
}

#[test]
fn old_name_is_gone() {
    for src in [
        include_str!("../src/lib.rs"),
        include_str!("../src/parser.rs"),
        include_str!("../src/report.rs"),
        include_str!("../src/main.rs"),
    ] {
        assert!(!src.contains("parse_line"));
    }
}

use task::Span;
use task::report::{Entry, totals};

#[test]
fn formats_hours_minutes_seconds() {
    assert_eq!(Span::from_secs(3723).to_string(), "1h 02m 03s");
    assert_eq!(Span::from_secs(45).to_string(), "0h 00m 45s");
    assert_eq!(Span::from_secs(0).to_string(), "0h 00m 00s");
    assert_eq!(Span::from_secs(36_000 + 59 * 60 + 9).to_string(), "10h 59m 09s");
    assert_eq!(format!("{}", Span::from_minutes(125)), "2h 05m 00s");
}

#[test]
fn totals_unchanged() {
    let entries = vec![
        Entry { project: "a".into(), span: Span::from_secs(10) },
        Entry { project: "b".into(), span: Span::from_secs(5) },
        Entry { project: "a".into(), span: Span::from_secs(20) },
    ];
    let t = totals(&entries);
    assert_eq!(t, vec![("a".to_string(), Span::from_secs(30)), ("b".to_string(), Span::from_secs(5))]);
}

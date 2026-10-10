use task::{Meeting, longest, minutes_between, total_minutes};

fn m(title: &str, start: u32, end: u32) -> Meeting {
    Meeting {
        title: title.into(),
        start,
        end,
    }
}

#[test]
fn totals() {
    assert_eq!(total_minutes(&[m("a", 0, 1440)]), 1440);
    assert_eq!(total_minutes(&[m("a", 600, 600), m("b", 610, 640)]), 30);
    assert_eq!(total_minutes(&[]), 0);
}

#[test]
fn helpers() {
    assert_eq!(minutes_between(100, 160), 60);
    assert_eq!(minutes_between(160, 100), 0);
    assert_eq!(longest(&[m("a", 0, 10), m("b", 0, 30)]), Some("b"));
}

use task::{Download, percent, status};

fn line(received: u64, size: u64) -> String {
    status(&Download {
        name: "f".into(),
        received,
        size,
    })
}

#[test]
fn various_progress() {
    assert_eq!(line(1, 3), "f 33%");
    assert_eq!(line(0, 50), "f 0%");
    assert_eq!(line(50, 50), "f 100%");
    assert_eq!(line(0, 0), "f 0%");
    assert_eq!(line(999, 1000), "f 99%");
}

#[test]
fn percent_itself() {
    assert_eq!(percent(1, 4), 25);
    assert_eq!(percent(3, 0), 0);
}

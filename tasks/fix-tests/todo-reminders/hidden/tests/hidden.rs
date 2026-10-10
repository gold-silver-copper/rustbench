use task::{Task, needs_reminder};

fn task(due_day: u32, done: bool) -> Task {
    Task {
        title: "t".into(),
        due_day,
        done,
    }
}

#[test]
fn open_tasks() {
    assert!(needs_reminder(&task(3, false), 10));
    assert!(needs_reminder(&task(11, false), 10));
    assert!(!needs_reminder(&task(12, false), 10));
}

#[test]
fn done_tasks_never() {
    assert!(!needs_reminder(&task(3, true), 10));
    assert!(!needs_reminder(&task(10, true), 10));
}

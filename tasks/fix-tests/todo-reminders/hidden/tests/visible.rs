use task::{Task, reminders};

fn task(title: &str, due_day: u32, done: bool) -> Task {
    Task {
        title: title.into(),
        due_day,
        done,
    }
}

#[test]
fn reminds_about_open_tasks_due_soon() {
    let tasks = [
        task("pay rent", 10, false),
        task("file taxes", 30, false),
        task("buy milk", 9, true),
    ];
    assert_eq!(reminders(&tasks, 10), vec!["pay rent"]);
}

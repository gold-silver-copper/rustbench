//! Reminders for a to-do app. Days are counted from an arbitrary epoch.

pub struct Task {
    pub title: String,
    pub due_day: u32,
    pub done: bool,
}

/// A task needs a reminder when it is not done and is due today, tomorrow, or overdue.
pub fn needs_reminder(task: &Task, today: u32) -> bool {
    if !task.done {
        return false;
    }
    task.due_day <= today + 1
}

pub fn reminders(tasks: &[Task], today: u32) -> Vec<&str> {
    tasks
        .iter()
        .filter(|t| needs_reminder(t, today))
        .map(|t| t.title.as_str())
        .collect()
}

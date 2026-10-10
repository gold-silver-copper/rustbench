use crate::task::Task;

/// A ready-made list for moving house.
pub fn moving_day() -> Vec<Task> {
    ["book a van", "pack the kitchen", "return the keys"]
        .iter()
        .map(|t| Task {
            title: t.to_string(),
            done: false,
            priority: 3,
        })
        .collect()
}

/// A finished example task, shown in the help screen.
pub fn example_done() -> Task {
    Task {
        title: "read the manual".to_string(),
        done: true,
        priority: 3,
    }
}

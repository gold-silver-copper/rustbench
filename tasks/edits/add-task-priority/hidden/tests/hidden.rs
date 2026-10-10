use task::{Task, TodoList};

#[test]
fn new_task_has_priority_three() {
    let t = Task::new("buy milk");
    assert_eq!(t.priority, 3);
    assert!(!t.done);
}

#[test]
fn literals_carry_priority() {
    let mut l = TodoList::default();
    l.add(Task {
        title: "fix bike".to_string(),
        done: false,
        priority: 1,
    });
    l.add(Task::new("fix bike"));
    assert!(l.complete("fix bike"));
    assert_eq!(l.remaining().len(), 1);
    assert_eq!(l.remaining()[0].priority, 3);
}

#[test]
fn templates_use_three() {
    let tasks = task::templates::moving_day();
    assert_eq!(tasks.len(), 3);
    assert!(tasks.iter().all(|t| t.priority == 3 && !t.done));
    let ex = task::templates::example_done();
    assert!(ex.done);
    assert_eq!(ex.priority, 3);
}

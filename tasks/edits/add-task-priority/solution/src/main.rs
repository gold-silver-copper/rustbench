use task::{Task, TodoList};

fn main() {
    let mut list = TodoList::default();
    for t in task::templates::moving_day() {
        list.add(t);
    }
    list.add(Task::new("call the bank"));
    list.add(Task {
        title: "cancel internet".to_string(),
        done: false,
        priority: 3,
    });
    list.complete("book a van");
    for t in list.remaining() {
        println!("[ ] {}", t.title);
    }
    println!("example: {}", task::templates::example_done().title);
}

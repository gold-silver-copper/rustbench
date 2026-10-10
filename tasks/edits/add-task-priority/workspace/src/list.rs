use crate::task::Task;

#[derive(Debug, Default)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Mark the first open task with this title done.
    pub fn complete(&mut self, title: &str) -> bool {
        match self.tasks.iter_mut().find(|t| t.title == title && !t.done) {
            Some(t) => {
                t.complete();
                true
            }
            None => false,
        }
    }

    pub fn remaining(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| !t.done).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completes_once() {
        let mut l = TodoList::default();
        l.add(Task {
            title: "water plants".to_string(),
            done: false,
        });
        assert!(l.complete("water plants"));
        assert!(!l.complete("water plants"));
        assert!(l.remaining().is_empty());
    }
}

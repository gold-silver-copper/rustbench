#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub title: String,
    pub done: bool,
    pub priority: u8,
}

impl Task {
    pub fn new(title: &str) -> Task {
        Task {
            title: title.to_string(),
            done: false,
            priority: 3,
        }
    }

    pub fn complete(&mut self) {
        self.done = true;
    }
}

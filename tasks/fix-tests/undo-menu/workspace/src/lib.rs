//! Edit history for a drawing app.

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Draw(String),
    Erase(String),
    Move { shape: String, dx: i32, dy: i32 },
}

#[derive(Default)]
pub struct History {
    actions: Vec<Action>,
}

impl History {
    pub fn record(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// The last `n` actions, most recent first (for the Undo menu).
    pub fn recent(&self, n: usize) -> Vec<&Action> {
        self.actions.iter().take(n).collect()
    }
}

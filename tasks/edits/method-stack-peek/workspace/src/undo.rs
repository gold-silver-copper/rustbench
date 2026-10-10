use crate::stack::Stack;

/// A text buffer with undo.
pub struct Editor {
    pub text: String,
    history: Stack<String>,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text: String::new(),
            history: Stack::new(),
        }
    }

    pub fn type_text(&mut self, s: &str) {
        self.history.push(self.text.clone());
        self.text.push_str(s);
    }

    pub fn undo(&mut self) -> bool {
        match self.history.pop() {
            Some(prev) => {
                self.text = prev;
                true
            }
            None => false,
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

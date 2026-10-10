//! The "recently opened files" list of a text editor: a fixed-size ring.

pub struct Recent {
    slots: Vec<Option<String>>,
    next: usize,
}

impl Recent {
    /// A list that remembers the last `capacity` files (at least one).
    pub fn new(capacity: usize) -> Self {
        Recent {
            slots: vec![None; capacity.max(1)],
            next: 0,
        }
    }

    /// Remembers `path`, forgetting the oldest file when the list is full.
    pub fn open(&mut self, path: &str) {
        let cap = self.slots.len();
        if let Some(slot) = self.slots.get_mut(self.next) {
            *slot = Some(path.to_string());
        }
        self.next += 1;
        if self.next >= cap - 1 {
            self.next = 0;
        }
    }

    /// Remembered files, oldest first.
    pub fn files(&self) -> Vec<&str> {
        let cap = self.slots.len();
        (0..cap)
            .filter_map(|i| self.slots.get((self.next + i) % cap)?.as_deref())
            .collect()
    }
}

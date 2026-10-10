pub struct Counter {
    pub page: String,
    pub visits: u32,
}

impl Counter {
    pub fn new(page: &str) -> Counter {
        Counter { page: page.to_string(), visits: 0 }
    }

    /// Records one visit and returns the new count.
    pub fn visit(&self) -> u32 {
        self.visits += 1;
        self.visits
    }
}

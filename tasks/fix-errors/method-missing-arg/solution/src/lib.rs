#[derive(Default)]
pub struct Stock {
    items: Vec<(String, u32)>,
}

impl Stock {
    /// Adds `qty` units of `name`, merging with an existing entry.
    pub fn add(&mut self, name: &str, qty: u32) {
        match self.items.iter_mut().find(|(n, _)| n == name) {
            Some((_, q)) => *q += qty,
            None => self.items.push((name.to_string(), qty)),
        }
    }

    /// Adds every name on a delivery note, each with the same quantity.
    pub fn receive(&mut self, names: &[&str], qty_each: u32) {
        for name in names {
            self.add(name, qty_each);
        }
    }

    pub fn quantity(&self, name: &str) -> u32 {
        self.items.iter().find(|(n, _)| n == name).map_or(0, |(_, q)| *q)
    }
}

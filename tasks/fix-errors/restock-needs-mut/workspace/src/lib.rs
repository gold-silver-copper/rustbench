pub struct Shelf {
    pub counts: Vec<u32>,
    pub capacity: u32,
}

impl Shelf {
    /// Fills one slot to capacity; returns how many units were added.
    pub fn restock(&mut self, slot: usize) -> u32 {
        match self.counts.get_mut(slot) {
            Some(c) => {
                let added = self.capacity.saturating_sub(*c);
                *c += added;
                added
            }
            None => 0,
        }
    }

    /// Fills every slot; returns the total units added.
    pub fn restock_all(&self) -> u32 {
        (0..self.counts.len()).map(|i| self.restock(i)).sum()
    }
}

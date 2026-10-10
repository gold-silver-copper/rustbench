use crate::Item;

/// Everything in stock.
#[derive(Debug, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    /// Add stock, merging with an existing item of the same SKU.
    pub fn receive(&mut self, item: Item) {
        match self.items.iter_mut().find(|i| i.sku == item.sku) {
            Some(existing) => existing.quantity += item.quantity,
            None => self.items.push(item),
        }
    }

    /// Take `n` units of `sku` off the shelf. Fails if there aren't enough.
    pub fn sell(&mut self, sku: &str, n: u32) -> Result<(), String> {
        let item = self
            .items
            .iter_mut()
            .find(|i| i.sku == sku)
            .ok_or_else(|| format!("no such item {sku}"))?;
        if item.quantity < n {
            return Err(format!("only {} of {sku} left", item.quantity));
        }
        item.quantity -= n;
        Ok(())
    }

    /// Total number of units in stock.
    pub fn units(&self) -> u64 {
        self.items.iter().map(|i| u64::from(i.quantity)).sum()
    }

    /// What all the stock is worth, in cents.
    pub fn total_value(&self) -> u64 {
        self.items
            .iter()
            .map(|i| i.unit_price_cents * u64::from(i.quantity))
            .sum()
    }
}

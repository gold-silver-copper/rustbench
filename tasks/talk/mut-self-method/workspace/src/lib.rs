use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Inventory {
    stock: BTreeMap<String, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory::default()
    }

    pub fn count(&self, sku: &str) -> u32 {
        self.stock.get(sku).copied().unwrap_or(0)
    }

    pub fn restock(&mut self, sku: &str, units: u32) {
        *self.stock.entry(sku.to_string()).or_insert(0) += units;
    }

    pub fn is_empty(&self) -> bool {
        self.stock.values().all(|&n| n == 0)
    }

    pub fn total_units(&self) -> u32 {
        self.stock.values().sum()
    }

    pub fn into_report(self) -> String {
        self.stock
            .into_iter()
            .map(|(sku, n)| format!("{sku}: {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Moves all of `other`'s stock into `into`.
pub fn merge(into: &mut Inventory, other: Inventory) {
    for (sku, n) in other.stock {
        *into.stock.entry(sku).or_insert(0) += n;
    }
}

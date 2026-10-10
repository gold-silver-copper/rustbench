use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PriceList {
    prices: HashMap<String, u32>,
}

impl PriceList {
    pub fn set(&mut self, sku: &str, cents: u32) {
        self.prices.insert(sku.to_string(), cents);
    }

    pub fn try_price(&self, sku: &str) -> Option<u32> {
        self.prices.get(sku).copied()
    }

    pub fn price_of(&self, sku: &str) -> u32 {
        self.prices[sku]
    }

    pub fn total(&self, skus: &[&str]) -> u32 {
        skus.iter().filter_map(|s| self.try_price(s)).sum()
    }

    pub fn discounted(&self, sku: &str, percent: u32) -> Option<u32> {
        let p = self.try_price(sku)?;
        Some(p - p * percent.min(100) / 100)
    }
}

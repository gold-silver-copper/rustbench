#[derive(Debug, Clone, PartialEq)]
pub struct StockItem {
    pub sku: String,
    pub name: String,
    pub quantity: u32,
    pub unit_price_cents: u64,
}

impl StockItem {
    pub fn new(sku: &str, name: &str, quantity: u32, unit_price_cents: u64) -> StockItem {
        StockItem {
            sku: sku.to_string(),
            name: name.to_string(),
            quantity,
            unit_price_cents,
        }
    }

    pub fn stock_value_cents(&self) -> u64 {
        self.quantity as u64 * self.unit_price_cents
    }
}

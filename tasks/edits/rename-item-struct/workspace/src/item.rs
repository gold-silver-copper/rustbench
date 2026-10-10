#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub sku: String,
    pub name: String,
    pub quantity: u32,
    pub unit_price_cents: u64,
}

impl Item {
    pub fn new(sku: &str, name: &str, quantity: u32, unit_price_cents: u64) -> Item {
        Item {
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

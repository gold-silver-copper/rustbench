/// A product on the shelf.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub sku: String,
    pub unit_price_cents: u64,
    pub quantity: u32,
}

impl Item {
    pub fn new(sku: &str, unit_price_cents: u64, quantity: u32) -> Item {
        Item { sku: sku.to_string(), unit_price_cents, quantity }
    }
}

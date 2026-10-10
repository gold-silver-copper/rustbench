#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct LineItem {
    pub sku: String,
    pub quantity: u32,
    pub unit_cents: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: OrderId,
    pub customer: String,
    pub items: Vec<LineItem>,
    pub notes: Option<String>,
}

impl Order {
    pub fn total_cents(&self) -> u64 {
        self.items
            .iter()
            .map(|i| i.unit_cents * u64::from(i.quantity))
            .sum()
    }
}

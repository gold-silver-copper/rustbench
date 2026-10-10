/// Reorder level used when a line doesn't give one.
pub const DEFAULT_REORDER_LEVEL: u32 = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub sku: String,
    pub quantity: u32,
    /// Reorder when the quantity drops to this level or below.
    pub reorder_level: u32,
}

impl Item {
    pub fn needs_reorder(&self) -> bool {
        self.quantity <= self.reorder_level
    }
}

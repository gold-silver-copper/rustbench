//! Reorder suggestions for a small shop's inventory.

pub struct Item {
    pub sku: &'static str,
    pub on_hand: u32,
    pub reorder_at: u32,
    pub discontinued: bool,
}

/// SKUs to reorder: items at or below their reorder point that are still sold.
pub fn to_reorder(items: &[Item]) -> Vec<&'static str> {
    items
        .iter()
        .filter(|i| i.on_hand <= i.reorder_at && i.discontinued)
        .map(|i| i.sku)
        .collect()
}

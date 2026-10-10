use crate::item::{DEFAULT_REORDER_LEVEL, Item};

/// Parses "SKU QUANTITY [REORDER_LEVEL]", e.g. "BOLT-10 42" or "BOLT-10 42 20".
pub fn parse_line(line: &str) -> Option<Item> {
    let mut parts = line.split_whitespace();
    let sku = parts.next()?.to_string();
    let quantity = parts.next()?.parse().ok()?;
    let reorder_level = match parts.next() {
        Some(level) => level.parse().ok()?,
        None => DEFAULT_REORDER_LEVEL,
    };
    Some(Item { sku, quantity })
}

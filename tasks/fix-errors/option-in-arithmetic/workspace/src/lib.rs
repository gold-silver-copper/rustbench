use std::collections::HashMap;

/// Cost of buying `qty` of `item`, or None if the item has no price.
pub fn cost(prices: &HashMap<String, u32>, item: &str, qty: u32) -> Option<u32> {
    let price = prices.get(item);
    Some(price * qty)
}

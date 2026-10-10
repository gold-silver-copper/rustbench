/// Names of items with fewer than `min` units in stock, in input order.
pub fn below_threshold(stock: &[(&str, u32)], min: u32) -> Vec<String> {
    stock
        .iter()
        .filter(|(_, qty)| *qty < min)
        .map(|(name, _)| name.to_string())
        .collect()
}

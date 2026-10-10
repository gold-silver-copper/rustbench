/// One-line summary of a basket, e.g. "3 items, 1250 cents total, 416 cents average".
pub fn summary(prices: &[u32]) -> String {
    let total: u32 = prices.iter().sum();
    let count = prices.len() as u32;
    let total = format!("{total} cents total");
    let average = if count == 0 { 0 } else { total / count };
    format!("{count} items, {total}, {average} cents average")
}

/// Price after a percentage discount, plus a flat shipping fee. All in cents.
pub fn final_price(price: u32, discount_percent: u32, shipping: u32) -> u32 {
    price - price * discount_percent / 100 + shipping
}

/// Total for a cart where every item ships separately for the same fee.
pub fn cart_total(prices: &[u32], discount_percent: u32, shipping: u32) -> u32 {
    prices.iter().map(|&p| final_price(p, discount_percent, shipping)).sum()
}

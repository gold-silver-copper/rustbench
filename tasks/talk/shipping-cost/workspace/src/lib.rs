/// Shipping cost in cents for a parcel of `weight_grams`.
pub fn shipping_cost_cents(weight_grams: u32, express: bool) -> u32 {
    let base = match weight_grams {
        0..=500 => 450,
        501..=2000 => 800,
        _ => 800 + (weight_grams - 2000).div_ceil(1000) * 250,
    };
    if express { base + base / 2 } else { base }
}

/// A function adding `percent` tax to a price in cents (rounding down).
pub fn with_tax(percent: u32) -> impl Fn(u32) -> u32 {
    let factor = 100 + percent;
    |cents| cents * factor / 100
}

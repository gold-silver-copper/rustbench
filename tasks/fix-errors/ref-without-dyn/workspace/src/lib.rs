pub trait Discount {
    fn apply(&self, cents: u32) -> u32;
}

/// Takes a percentage off.
pub struct Percent(pub u32);

/// Takes a fixed amount off, never going below zero.
pub struct Flat(pub u32);

impl Discount for Percent {
    fn apply(&self, cents: u32) -> u32 {
        cents - cents * self.0 / 100
    }
}

impl Discount for Flat {
    fn apply(&self, cents: u32) -> u32 {
        cents.saturating_sub(self.0)
    }
}

/// Applies every discount in order.
pub fn checkout(cents: u32, discounts: &[&Discount]) -> u32 {
    discounts.iter().fold(cents, |c, d| d.apply(c))
}

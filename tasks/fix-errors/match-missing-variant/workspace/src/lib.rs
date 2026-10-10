#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shipping {
    Standard,
    Express,
    Overnight,
    Pickup,
}

/// Fee in cents: standard 499, express 999, overnight 1999, pickup free.
pub fn fee(method: Shipping) -> u32 {
    match method {
        Shipping::Standard => 499,
        Shipping::Express => 999,
        Shipping::Overnight => 1999,
    }
}

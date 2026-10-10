//! Comparing supplier offers.

pub struct Offer {
    pub supplier: &'static str,
    pub cents: u32,
}

/// The supplier with the lowest price (the first one listed on a tie), or
/// `None` with no offers.
pub fn cheapest(offers: &[Offer]) -> Option<&'static str> {
    let mut best = None;
    let mut best_price = 0;
    for o in offers {
        if o.cents < best_price {
            best = Some(o.supplier);
            best_price = o.cents;
        }
    }
    best
}

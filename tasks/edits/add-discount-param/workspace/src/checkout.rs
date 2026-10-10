use crate::cart::{self, LineItem};

/// Flat shipping fee in cents, waived at or above the free-shipping threshold.
pub fn shipping(items: &[LineItem]) -> u64 {
    if cart::total(items) >= 5000 { 0 } else { 499 }
}

/// Amount to charge, in cents.
pub fn amount_due(items: &[LineItem]) -> u64 {
    cart::total(items) + shipping(items)
}

/// One line per item plus the totals.
pub fn summary(items: &[LineItem]) -> String {
    let mut out = String::new();
    for it in items {
        out.push_str(&format!("{} x{}: {}\n", it.name, it.qty, it.subtotal()));
    }
    out.push_str(&format!("items: {}\n", cart::total(items)));
    out.push_str(&format!("shipping: {}\n", shipping(items)));
    out.push_str(&format!("due: {}\n", amount_due(items)));
    out
}

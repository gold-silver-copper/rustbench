/// Price including VAT, in cents, rounded to the nearest cent.
pub fn gross(net_cents: u64) -> u64 {
    (net_cents as f64 * (1.0 + 0.2)).round() as u64
}

/// The VAT part of a net price, in cents.
pub fn vat_of(net_cents: u64) -> u64 {
    (net_cents as f64 * 0.2).round() as u64
}

/// Net price from a VAT-inclusive price, in cents.
pub fn net_from_gross(gross_cents: u64) -> u64 {
    (gross_cents as f64 / (1.0 + 0.2)).round() as u64
}

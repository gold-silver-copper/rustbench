//! Invoice totals. Amounts are in cents; tax rates in basis points (1/100 of a percent).

pub struct Line {
    pub unit_cents: u64,
    pub qty: u64,
}

pub fn subtotal(lines: &[Line]) -> u64 {
    lines.iter().map(|l| l.unit_cents * l.qty).sum()
}

/// Tax on `cents` at `rate_bp`, rounded down to a whole cent.
pub fn tax(cents: u64, rate_bp: u64) -> u64 {
    cents * rate_bp / 10_000
}

/// What the customer pays: the subtotal plus tax.
pub fn total(lines: &[Line], rate_bp: u64) -> u64 {
    let sub = subtotal(lines);
    sub - tax(sub, rate_bp)
}

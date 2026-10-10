use crate::round_cents;

pub struct Line {
    pub unit_cents: i64,
    pub quantity: f64,
}

pub fn round_total(lines: &[Line]) -> i64 {
    round_cents(lines.iter().map(|l| l.unit_cents as f64 * l.quantity).sum())
}

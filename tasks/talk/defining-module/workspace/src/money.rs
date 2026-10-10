/// Rounds an amount in cents to the nearest whole cent, halves away from zero.
pub fn round_cents(amount: f64) -> i64 {
    amount.round() as i64
}

pub fn to_display(cents: i64) -> String {
    format!("{}.{:02}", cents / 100, (cents % 100).abs())
}

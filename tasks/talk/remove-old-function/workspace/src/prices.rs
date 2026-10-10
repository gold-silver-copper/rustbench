pub fn format_price(cents: u64, currency: &str) -> String {
    format!("{}.{:02} {currency}", cents / 100, cents % 100)
}

#[deprecated(note = "use format_price, which takes the currency")]
pub fn legacy_format_price(cents: u64) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

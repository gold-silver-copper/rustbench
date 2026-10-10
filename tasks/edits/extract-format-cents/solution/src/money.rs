/// Format an amount of cents as dollars: 1205 -> "$12.05", -340 -> "-$3.40".
pub fn format_cents(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let abs = cents.unsigned_abs();
    format!("{sign}${}.{:02}", abs / 100, abs % 100)
}

use crate::Line;

/// A multi-line invoice with a header and a total.
pub fn render_invoice(customer: &str, lines: &[Line]) -> String {
    let mut out = format!("INVOICE for {customer}\n");
    let mut total = 0;
    for line in lines {
        total += line.cents;
        let sign = if line.cents < 0 { "-" } else { "" };
        let abs = line.cents.unsigned_abs();
        out.push_str(&format!(
            "  {:<20}{:>12}\n",
            line.label,
            format!("{sign}${}.{:02}", abs / 100, abs % 100)
        ));
    }
    let sign = if total < 0 { "-" } else { "" };
    let abs = total.unsigned_abs();
    out.push_str(&format!(
        "  {:<20}{:>12}\n",
        "TOTAL",
        format!("{sign}${}.{:02}", abs / 100, abs % 100)
    ));
    out
}

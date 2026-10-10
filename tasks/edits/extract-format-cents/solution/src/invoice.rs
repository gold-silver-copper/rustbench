use crate::Line;
use crate::money::format_cents;

/// A multi-line invoice with a header and a total.
pub fn render_invoice(customer: &str, lines: &[Line]) -> String {
    let mut out = format!("INVOICE for {customer}\n");
    let mut total = 0;
    for line in lines {
        total += line.cents;
        out.push_str(&format!("  {:<20}{:>12}\n", line.label, format_cents(line.cents)));
    }
    out.push_str(&format!("  {:<20}{:>12}\n", "TOTAL", format_cents(total)));
    out
}

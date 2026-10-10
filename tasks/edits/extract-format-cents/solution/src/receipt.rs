use crate::Line;
use crate::money::format_cents;

/// A one-line receipt: "coffee $3.50, refund -$1.00 = $2.50".
pub fn render_receipt(lines: &[Line]) -> String {
    let parts: Vec<String> = lines
        .iter()
        .map(|l| format!("{} {}", l.label, format_cents(l.cents)))
        .collect();
    let total: i64 = lines.iter().map(|l| l.cents).sum();
    format!("{} = {}", parts.join(", "), format_cents(total))
}

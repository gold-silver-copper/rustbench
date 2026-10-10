use crate::Line;

/// A one-line receipt: "coffee $3.50, refund -$1.00 = $2.50".
pub fn render_receipt(lines: &[Line]) -> String {
    let parts: Vec<String> = lines
        .iter()
        .map(|l| {
            let sign = if l.cents < 0 { "-" } else { "" };
            let abs = l.cents.unsigned_abs();
            format!("{} {sign}${}.{:02}", l.label, abs / 100, abs % 100)
        })
        .collect();
    let total: i64 = lines.iter().map(|l| l.cents).sum();
    let sign = if total < 0 { "-" } else { "" };
    let abs = total.unsigned_abs();
    format!("{} = {sign}${}.{:02}", parts.join(", "), abs / 100, abs % 100)
}

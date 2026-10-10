use std::fmt::Write as _;

use crate::Row;

/// Write `rows` as an aligned two-column table with a header and a rule.
pub fn render_table(out: &mut String, rows: &[Row]) {
    let width = rows.iter().map(|r| r.name.len()).max().unwrap_or(0).max(4);
    let _ = writeln!(out, "{:<width$}  score", "name");
    let _ = writeln!(out, "{}", "-".repeat(width + 7));
    for r in rows {
        let _ = writeln!(out, "{:<width$}  {:>5}", r.name, r.score);
    }
}

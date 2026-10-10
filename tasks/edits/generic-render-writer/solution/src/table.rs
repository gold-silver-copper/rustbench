use std::fmt::Write;

use crate::Row;

/// Write `rows` as an aligned two-column table with a header and a rule.
pub fn render_table<W: Write>(out: &mut W, rows: &[Row]) -> std::fmt::Result {
    let width = rows.iter().map(|r| r.name.len()).max().unwrap_or(0).max(4);
    writeln!(out, "{:<width$}  score", "name")?;
    writeln!(out, "{}", "-".repeat(width + 7))?;
    for r in rows {
        writeln!(out, "{:<width$}  {:>5}", r.name, r.score)?;
    }
    Ok(())
}

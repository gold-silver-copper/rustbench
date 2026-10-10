#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DateFormat {
    /// 2024-03-09
    Iso,
    /// 03/09/2024
    Us,
    /// 09.03.2024
    European,
}

/// The format used by `export_row`.
pub const EXPORT_FORMAT: DateFormat = DateFormat::Iso;

pub fn format_date(y: u32, m: u32, d: u32, f: DateFormat) -> String {
    match f {
        DateFormat::Iso => format!("{y:04}-{m:02}-{d:02}"),
        DateFormat::Us => format!("{m:02}/{d:02}/{y:04}"),
        DateFormat::European => format!("{d:02}.{m:02}.{y:04}"),
    }
}

pub fn export_row(name: &str, date: (u32, u32, u32)) -> String {
    let (y, m, d) = date;
    format!("{name},{}", format_date(y, m, d, EXPORT_FORMAT))
}

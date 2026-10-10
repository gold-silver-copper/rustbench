//! Progress lines for a file downloader.

/// `part` as a whole percentage of `whole`, rounded down; 0 when `whole` is 0.
pub fn percent(part: u64, whole: u64) -> u64 {
    if whole == 0 { 0 } else { part * 100 / whole }
}

pub struct Download {
    pub name: String,
    pub received: u64,
    pub size: u64,
}

/// A status line like `photo.jpg 40%`.
pub fn status(d: &Download) -> String {
    format!("{} {}%", d.name, percent(d.received, d.size))
}

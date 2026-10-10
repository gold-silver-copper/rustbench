//! Star ratings for a recipe site, shown with one decimal.

/// The average of the ratings rounded to one decimal place (halves round up),
/// or `None` when there are none.
pub fn average(stars: &[u8]) -> Option<f64> {
    if stars.is_empty() {
        return None;
    }
    let sum: u32 = stars.iter().map(|&s| u32::from(s)).sum();
    let mean = f64::from(sum) / stars.len() as f64;
    Some((mean * 10.0).floor() / 10.0)
}

pub fn label(stars: &[u8]) -> String {
    match average(stars) {
        Some(a) => format!("{a:.1} stars"),
        None => "no ratings yet".to_string(),
    }
}

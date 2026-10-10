use crate::color::{self, Rgb};

/// Parse a comma-separated list of colors, e.g. "#ff0000, #00ff00".
pub fn parse_palette(text: &str) -> Vec<Rgb> {
    text.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(color::parse_hex)
        .collect()
}

/// The palette sorted from darkest to lightest.
pub fn by_brightness(mut colors: Vec<Rgb>) -> Vec<Rgb> {
    colors.sort_by_key(|c| c.luma());
    colors
}

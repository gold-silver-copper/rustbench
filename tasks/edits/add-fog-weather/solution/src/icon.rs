use crate::Weather;

/// A one-character icon for terminals without emoji.
pub fn icon(w: Weather) -> &'static str {
    match w {
        Weather::Sunny => "*",
        Weather::Rainy => "/",
        Weather::Snowy => "#",
        Weather::Fog => "~",
    }
}

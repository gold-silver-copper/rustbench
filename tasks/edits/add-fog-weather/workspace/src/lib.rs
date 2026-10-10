//! Turning station reports into a short forecast line.

pub mod advice;
pub mod icon;

/// What the sky is doing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weather {
    Sunny,
    Rainy,
    Snowy,
}

impl Weather {
    /// Parse a two-letter station code.
    pub fn from_code(code: &str) -> Option<Weather> {
        match code {
            "SU" => Some(Weather::Sunny),
            "RA" => Some(Weather::Rainy),
            "SN" => Some(Weather::Snowy),
            _ => None,
        }
    }
}

/// A one-line forecast for a station code, e.g. `* sunglasses`.
pub fn forecast(code: &str) -> String {
    match Weather::from_code(code) {
        Some(w) => format!("{} {}", icon::icon(w), advice::advice(w)),
        None => format!("? unknown code {code}"),
    }
}

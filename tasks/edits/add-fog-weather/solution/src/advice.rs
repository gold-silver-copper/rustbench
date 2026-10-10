use crate::Weather;

/// What to wear or do.
pub fn advice(w: Weather) -> &'static str {
    match w {
        Weather::Sunny => "sunglasses",
        Weather::Rainy => "umbrella",
        Weather::Snowy => "warm boots",
        Weather::Fog => "drive carefully",
    }
}

/// Whether it is a good day to cycle to work.
pub fn good_for_cycling(w: Weather) -> bool {
    match w {
        Weather::Sunny => true,
        Weather::Rainy | Weather::Snowy | Weather::Fog => false,
    }
}

pub mod temperature {
    /// Celsius to Fahrenheit.
    fn to_fahrenheit(celsius: f64) -> f64 {
        celsius * 9.0 / 5.0 + 32.0
    }

    pub fn is_freezing(celsius: f64) -> bool {
        celsius <= 0.0
    }
}

/// Forecast line such as "Mon: 68F".
pub fn forecast_line(day: &str, celsius: f64) -> String {
    format!("{day}: {:.0}F", temperature::to_fahrenheit(celsius))
}

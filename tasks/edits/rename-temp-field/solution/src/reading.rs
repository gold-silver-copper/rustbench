#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reading {
    /// Hour of the day, 0..24.
    pub hour: u8,
    /// Degrees Celsius.
    pub celsius: f64,
    /// Relative humidity in percent.
    pub humidity: f64,
}

impl Reading {
    pub fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    /// Parse `hour,celsius,humidity`, e.g. `14,22.5,40`.
    pub fn parse(s: &str) -> Option<Reading> {
        let mut it = s.split(',').map(str::trim);
        let hour = it.next()?.parse().ok()?;
        let celsius = it.next()?.parse().ok()?;
        let humidity = it.next()?.parse().ok()?;
        Some(Reading { hour, celsius, humidity })
    }
}

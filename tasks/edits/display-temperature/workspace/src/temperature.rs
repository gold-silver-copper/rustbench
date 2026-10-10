#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature {
    pub celsius: f64,
}

impl Temperature {
    pub fn from_celsius(celsius: f64) -> Temperature {
        Temperature { celsius }
    }

    pub fn from_fahrenheit(f: f64) -> Temperature {
        Temperature { celsius: (f - 32.0) * 5.0 / 9.0 }
    }

    pub fn is_freezing(self) -> bool {
        self.celsius <= 0.0
    }
}

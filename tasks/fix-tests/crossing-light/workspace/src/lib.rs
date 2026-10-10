//! The controller for a pedestrian crossing light.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Green,
    Yellow,
    Red,
}

impl Light {
    /// The light that follows this one: green, yellow, red, then green again.
    pub fn next(self) -> Light {
        match self {
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Green,
            Light::Red => Light::Green,
        }
    }

    /// Seconds this light stays on.
    pub fn duration(self) -> u32 {
        match self {
            Light::Green => 30,
            Light::Yellow => 4,
            Light::Red => 20,
        }
    }
}

/// The light showing `secs` seconds after green came on.
pub fn light_at(secs: u32) -> Light {
    let mut light = Light::Green;
    let mut left = secs;
    while left >= light.duration() {
        left -= light.duration();
        light = light.next();
    }
    light
}

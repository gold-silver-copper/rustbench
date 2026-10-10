/// Lowest and highest temperatures the heating may aim for, in °C.
pub const MIN_TARGET: f32 = 10.0;
pub const MAX_TARGET: f32 = 30.0;

/// Holds the room temperature and the temperature we're heating towards.
#[derive(Debug, Clone, PartialEq)]
pub struct Thermostat {
    target: f32,
    current: f32,
}

impl Thermostat {
    pub fn new(current: f32) -> Thermostat {
        Thermostat { target: 20.0, current }
    }

    /// Set the target, clamped to `MIN_TARGET..=MAX_TARGET`.
    pub fn set_target(&mut self, t: f32) {
        self.target = t.clamp(MIN_TARGET, MAX_TARGET);
    }

    pub fn target(&self) -> f32 {
        self.target
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn measure(&mut self, current: f32) {
        self.current = current;
    }

    /// Whether the heater should be running (half a degree of slack).
    pub fn heating(&self) -> bool {
        self.current < self.target - 0.5
    }
}

use crate::entities::Vec2;

pub struct Camera {
    pub focus: Vec2,
    pub zoom: f32,
    /// How quickly the camera catches up with its focus (not a velocity).
    pub follow_rate: f32,
}

impl Camera {
    pub fn step(&mut self, target: Vec2) {
        self.focus.0 += (target.0 - self.focus.0) * self.follow_rate;
        self.focus.1 += (target.1 - self.focus.1) * self.follow_rate;
    }
}

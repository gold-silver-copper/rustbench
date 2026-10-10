pub type Vec2 = (f32, f32);

pub struct Player {
    pub name: String,
    pub position: Vec2,
    pub velocity: Vec2,
    pub health: u32,
}

pub struct Enemy {
    pub kind: String,
    pub position: Vec2,
    pub vel: Vec2,
}

pub struct Projectile {
    pub position: Vec2,
    pub speed: f32,
    pub heading: f32,
}

impl Projectile {
    /// The velocity this projectile moves with, from speed and heading.
    pub fn velocity(&self) -> Vec2 {
        (self.speed * self.heading.cos(), self.speed * self.heading.sin())
    }
}

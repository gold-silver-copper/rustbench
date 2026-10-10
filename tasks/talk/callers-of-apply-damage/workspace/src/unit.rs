#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub name: String,
    pub hp: i32,
    pub armor: i32,
}

impl Unit {
    pub fn new(name: &str, hp: i32, armor: i32) -> Self {
        Unit {
            name: name.to_string(),
            hp,
            armor,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

/// Lowers `target.hp` by `amount` minus armor; armor never heals.
pub fn apply_damage(target: &mut Unit, amount: i32) {
    let dealt = (amount - target.armor).max(0);
    target.hp -= dealt;
}

pub fn heal(target: &mut Unit, amount: i32) {
    // Healing is not a negative apply_damage: armor must not reduce it.
    target.hp += amount;
}

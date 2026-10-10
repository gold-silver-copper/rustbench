use crate::unit::{Unit, apply_damage};

pub fn melee_attack(attacker: &Unit, target: &mut Unit) {
    let power = attacker.hp / 10 + 5;
    apply_damage(target, power);
}

pub fn ranged_attack(target: &mut Unit, distance: u32) {
    let power = 12 - distance.min(10) as i32;
    if power > 0 {
        apply_damage(target, power);
    }
}

pub fn describe_hit(target: &Unit) -> String {
    format!("{} was hit (see apply_damage for the formula)", target.name)
}

use crate::unit::Unit;

pub fn fire_trap(units: &mut [Unit]) {
    for u in units.iter_mut() {
        crate::unit::apply_damage(u, 8);
    }
}

pub fn poison_tick(u: &mut Unit) {
    // Poison ignores armor, so it does not go through apply_damage.
    u.hp -= 2;
}

/// Adds `bonus` points to every score, capping at 100.
pub fn apply_curve(scores: &mut [u32], bonus: u32) {
    for s in scores.iter_mut() {
        *s = (*s + bonus).min(100);
    }
}

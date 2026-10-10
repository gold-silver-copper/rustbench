/// A badge line for every guest, plus how many guests there were.
pub fn badges(guests: Vec<String>) -> (Vec<String>, usize) {
    let mut out = Vec::new();
    for g in &guests {
        out.push(format!("HELLO, my name is {g}"));
    }
    (out, guests.len())
}

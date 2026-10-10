fn main() {
    let x = (0, 2);
    match x {
        (0, y) | (y, 0) => { /* use y */}
        _ => {}
    }
}

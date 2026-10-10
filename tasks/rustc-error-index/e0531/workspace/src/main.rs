fn main() {
    let Type(x) = Type(12);
    match Bar(12) {
        Bar(x) => {}
        _ => {}
    }
}

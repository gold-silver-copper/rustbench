fn main() {
    let x: for<'a> fn(&'a i32) -> &'a i32;
    let y: fn() -> &'static i32;
}

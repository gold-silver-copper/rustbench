fn main() {
    static TEST: i32 = 0;

    let r = 123;
    match r {
        TEST => {}
    }
}

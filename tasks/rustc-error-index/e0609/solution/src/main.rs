fn main() {
    struct StructWithFields {
        x: u32,
    }

    let s = StructWithFields { x: 0 };
    println!("{}", s.x);
}

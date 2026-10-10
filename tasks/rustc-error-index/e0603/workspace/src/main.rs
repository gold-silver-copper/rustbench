fn main() {
    mod foo {
        const PRIVATE: u32 = 0x_a_bad_1dea_u32;
    }

    println!("const value: {}", foo::PRIVATE);
}

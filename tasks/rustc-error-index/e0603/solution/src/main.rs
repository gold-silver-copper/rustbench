fn main() {
    mod foo {
        pub const PRIVATE: u32 = 0x_a_bad_1dea_u32;
    }

    println!("const value: {}", foo::PRIVATE);
}

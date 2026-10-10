fn main() {
    mod bar {
        pub struct Foo {
            pub a: isize,
            pub b: isize,
        }
    }

    let f = bar::Foo{ a: 0, b: 0 };
}

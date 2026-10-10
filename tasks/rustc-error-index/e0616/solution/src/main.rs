fn main() {
    mod some_module {
        pub struct Foo {
            pub x: u32,
        }

        impl Foo {
            pub fn new() -> Foo { Foo { x: 0 } }
        }
    }

    let f = some_module::Foo::new();
    println!("{}", f.x);
}

fn main() {
    mod inner {
        pub struct Foo;

        impl Foo {
            pub fn method(&self) {}
        }
    }

    let foo = inner::Foo;
    foo.method();
}

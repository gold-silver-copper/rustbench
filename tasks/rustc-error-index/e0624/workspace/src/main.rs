fn main() {
    mod inner {
        pub struct Foo;

        impl Foo {
            fn method(&self) {}
        }
    }

    let foo = inner::Foo;
    foo.method();
}

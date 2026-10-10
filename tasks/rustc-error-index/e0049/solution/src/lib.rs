trait Foo {
    fn foo<T: Default>(x: T) -> Self;
}

struct Bar;

impl Foo for Bar {
    fn foo<T: Default>(x: T) -> Self {
        Bar
    }
}

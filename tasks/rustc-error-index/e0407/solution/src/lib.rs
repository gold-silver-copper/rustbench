trait Foo {
    fn a();
}

struct Bar;

impl Foo for Bar {
    fn a() {}
}

impl Bar {
    fn b() {}
}

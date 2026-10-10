struct Bar;

trait Foo {
    type N;
}

impl Foo for Bar {
    type N = u32;
}

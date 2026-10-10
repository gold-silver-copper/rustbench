struct Bar;

trait Foo {
    const N : u32;
}

impl Foo for Bar {
    type N = u32;
}

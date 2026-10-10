trait Foo {
    type N;
}

struct Bar;

impl Foo for Bar {
    const N : u32 = 0;
}

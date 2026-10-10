trait Foo {
    fn foo(&self, x: u8) -> bool;
}

struct Bar;

impl Foo for Bar {
    fn foo(&self) -> bool { true }
}

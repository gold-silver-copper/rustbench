struct Foo;
struct Bar;

trait Trait {
    fn foo(&self);
}

impl Trait for Foo {
    fn foo(self: &Bar) {}
}

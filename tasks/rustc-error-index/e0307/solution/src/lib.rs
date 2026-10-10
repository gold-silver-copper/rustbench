struct Foo;
trait Trait {
    fn foo(&self);
}
impl Trait for Foo {
    fn foo(self: &Foo) {}
}

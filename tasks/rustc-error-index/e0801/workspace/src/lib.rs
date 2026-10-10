struct Foo;

impl Foo {
    fn foo(self: impl std::ops::Deref<Target=Self>) {}
}

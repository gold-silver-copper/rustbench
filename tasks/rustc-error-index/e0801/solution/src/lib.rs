struct Foo;

impl Foo {
    fn foo(self) {}
    fn foo2(self: std::rc::Rc<Self>) {}
}

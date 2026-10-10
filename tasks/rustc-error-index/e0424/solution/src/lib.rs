struct Foo;

impl Foo {
    fn bar(&self) {}

    fn foo(self) {
        self.bar();
    }
}

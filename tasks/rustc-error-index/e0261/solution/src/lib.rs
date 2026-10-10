struct Foo<'a> {
    x: &'a str,
}

impl<'a> Foo<'a> {
    fn foo(x: &'a str) {}
}

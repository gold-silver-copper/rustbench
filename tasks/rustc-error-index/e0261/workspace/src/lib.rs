struct Foo<'a> {
    x: &'a str,
}

impl Foo<'a> {
    fn foo<'a>(x: &'a str) {}
}

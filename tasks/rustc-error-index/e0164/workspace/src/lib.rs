enum A {
    B,
    C,
}

impl A {
    fn new() {}
}

fn bar(foo: A) {
    match foo {
        A::new() => (),
        _ => {}
    }
}

struct Foo<'a> {
    x: fn(&'a i32),
}

trait Trait<'a, 'b> {
    type Out;
}

impl<'a, 'b> Trait<'a, 'b> for usize {
    type Out = &'a Foo<'b>;
}

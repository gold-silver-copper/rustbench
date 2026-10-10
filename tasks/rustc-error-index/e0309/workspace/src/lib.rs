struct Foo<'a, T> {
    foo: <T as SomeTrait<'a>>::Output,
}

trait SomeTrait<'a> {
    type Output;
}

impl<'a, T> SomeTrait<'a> for T
where
    T: 'a,
{
    type Output = u32;
}

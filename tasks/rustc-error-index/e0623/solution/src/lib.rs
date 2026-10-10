struct Foo<'a, 'b, T>(std::marker::PhantomData<(&'a (), &'b (), T)>)
where
    T: Convert<'a, 'b>;

trait Convert<'a, 'b>: Sized {
    fn cast(&'a self) -> &'b Self;
}
impl<'long: 'short, 'short, T> Convert<'long, 'short> for T {
    fn cast(&'long self) -> &'short T {
        self
    }
}
fn badboi<'in_: 'out, 'out, T>(
    x: Foo<'in_, 'out, T>,
    sadness: &'in_ T
) -> &'out T {
    sadness.cast()
}

pub trait Foo<T> {
    type A;

    fn get(&self, t: T) -> Self::A;
}

fn foo2<I : for<'x> Foo<&'x isize>>(
    field: I::A) {}

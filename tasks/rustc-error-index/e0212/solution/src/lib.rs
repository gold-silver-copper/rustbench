pub trait Foo<T> {
    type A;

    fn get(&self, t: T) -> Self::A;
}

fn foo3<I : for<'x> Foo<&'x isize>>(
    x: <I as Foo<&isize>>::A) {}

fn foo4<'a, I : for<'x> Foo<&'x isize>>(
    x: <I as Foo<&'a isize>>::A) {}

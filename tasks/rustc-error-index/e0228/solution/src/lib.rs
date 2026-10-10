trait Trait { }

struct TwoBounds<'a, 'b, T: Sized + 'a + 'b> {
    x: &'a i32,
    y: &'b i32,
    z: T,
}

type Foo<'a, 'b> = TwoBounds<'a, 'b, dyn Trait + 'b>;

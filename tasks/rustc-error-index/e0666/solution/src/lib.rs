trait MyGenericTrait<T> {}
trait MyInnerTrait {}

fn foo<T: MyInnerTrait>(
    bar: impl MyGenericTrait<T>,
) {}

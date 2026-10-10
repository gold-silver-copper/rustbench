trait MyGenericTrait<T> {}
trait MyInnerTrait {}

fn foo(
    bar: impl MyGenericTrait<impl MyInnerTrait>,
) {}

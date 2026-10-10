trait Trait {
    type Bar;
}

type Foo = dyn Trait<Bar=i32>;

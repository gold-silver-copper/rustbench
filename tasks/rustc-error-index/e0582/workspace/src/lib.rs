trait Foo {
    type Assoc<'a>;
}

struct Bar<X, F>
where
    X: Foo,
    F: for<'a> Fn(X::Assoc<'a>) -> &'a i32
{
    x: X,
    f: F
}

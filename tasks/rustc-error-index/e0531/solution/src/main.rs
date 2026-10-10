fn main() {
    struct Type(u32);

    enum Foo {
        Bar(u32),
    }
    use Foo::*;

    let Type(x) = Type(12);
    match Type(12) {
        Type(x) => {}
        _ => {}
    }
}

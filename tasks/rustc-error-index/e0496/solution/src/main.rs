struct Foo<'a> {
    a: &'a i32,
}

impl<'a> Foo<'a> {
    fn f<'b>(x: &'b i32) {
    }
}

fn main() {
}

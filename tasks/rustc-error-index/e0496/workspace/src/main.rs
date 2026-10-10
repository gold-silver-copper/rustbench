fn main() {
    struct Foo<'a> {
        a: &'a i32,
    }

    impl<'a> Foo<'a> {
        fn f<'a>(x: &'a i32) {
        }
    }
}

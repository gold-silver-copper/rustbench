fn main() {
    struct Foo { x: u32 }
    impl Foo { fn method(&self) {} }
    let f = Foo { x: 0 };
    f.method();
}

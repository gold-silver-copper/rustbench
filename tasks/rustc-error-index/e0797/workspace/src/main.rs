fn main() {
    struct Foo {
        fizz: u8,
        buzz: u8
    }

    let f1 = Foo { fizz: 10, buzz: 1};
    let f2 = Foo { fizz: 10, .. };
}

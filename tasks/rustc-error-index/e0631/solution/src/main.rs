fn foo<F: Fn(i32)>(f: F) {
}

fn main() {
    foo(|x: i32| {});
}

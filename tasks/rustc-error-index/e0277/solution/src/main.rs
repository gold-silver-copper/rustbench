use std::fmt;

fn some_func<T: fmt::Debug>(foo: T) {
    println!("{:?}", foo);
}

fn main() {
    some_func(5i32);

}

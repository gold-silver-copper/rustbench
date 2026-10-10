fn main() {
    enum E {
        A(i32),
    }

    let e = E::A(42);

    match e {
        E::A { number } => {
            println!("{}", number);
        }
    }
}

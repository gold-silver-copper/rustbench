fn main() {
    let r = &[1, 2, 3, 4];
    match r {
        &[a, b, ..] => {
            println!("a={}, b={}", a, b);
        }
    }
}

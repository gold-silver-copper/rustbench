fn main() {
    let r = &[1, 2, 3, 4, 5];
    match r {
        &[a, b, c, rest @ ..] => {
            println!("a={}, b={}, c={} rest={:?}", a, b, c, rest);
        }
    }
}

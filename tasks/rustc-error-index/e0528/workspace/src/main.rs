fn main() {
    let r = &[1, 2];
    match r {
        &[a, b, c, rest @ ..] => {
            println!("a={}, b={}, c={} rest={:?}", a, b, c, rest);
        }
    }
}

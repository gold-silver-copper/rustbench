fn main() {
    let mut home = task::Counter::new("home");
    home.visit();
    home.visit();
    let n = home.visit();
    println!("{}: {}", home.page, n);
}

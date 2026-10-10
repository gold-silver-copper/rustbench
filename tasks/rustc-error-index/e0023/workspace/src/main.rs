fn main() {
    enum Fruit {
        Apple(String, String),
        Pear(u32),
    }

    let x = Fruit::Apple(String::new(), String::new());

    match x {
        Fruit::Apple(a) => {},
        _ => {}
    }
}

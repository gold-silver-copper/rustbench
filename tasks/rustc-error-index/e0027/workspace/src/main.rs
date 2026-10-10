fn main() {
    struct Dog {
        name: String,
        age: u32,
    }

    let d = Dog { name: "Rusty".to_string(), age: 8 };

    match d {
        Dog { age: x } => {}
    }
}

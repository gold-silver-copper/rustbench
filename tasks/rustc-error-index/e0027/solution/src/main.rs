fn main() {
    struct Dog {
        name: String,
        age: u32,
    }

    let d = Dog { name: "Rusty".to_string(), age: 8 };

    match d {
        Dog { name: ref n, age: x } => {}
    }

    match d {
        Dog { age: x, .. } => {}
    }
}

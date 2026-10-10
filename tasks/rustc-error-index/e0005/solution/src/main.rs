fn main() {
    let x = Some(1);

    match x {
        Some(y) => {
        },
        None => {}
    }

    if let Some(y) = x {
    }
}

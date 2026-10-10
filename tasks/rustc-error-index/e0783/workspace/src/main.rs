fn main() {
    match 2u8 {
        0...9 => println!("Got a number less than 10"),
        _ => println!("Got a number 10 or more"),
    }
}

fn main() {
    let low = task::below_threshold(&[("bolts", 3), ("nuts", 40), ("washers", 7)], 10);
    // expected output: restock: ["bolts", "washers"]
    println!("restock: {}", low);
}

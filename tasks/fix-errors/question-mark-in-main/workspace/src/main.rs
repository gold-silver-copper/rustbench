fn main() {
    let total = task::parse_total("12, 7, 30")?;
    println!("total: {total}");
}

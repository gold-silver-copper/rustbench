fn main() -> Result<(), std::num::ParseIntError> {
    let total = task::parse_total("12, 7, 30")?;
    println!("total: {total}");
    Ok(())
}

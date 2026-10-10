use std::num::ParseIntError;

/// Sum of a comma-separated list of numbers.
pub fn parse_total(list: &str) -> Result<u32, ParseIntError> {
    let mut total = 0;
    for part in list.split(',') {
        total += part.trim().parse::<u32>()?;
    }
    Ok(total)
}

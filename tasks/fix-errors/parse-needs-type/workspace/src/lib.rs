/// Doubles a typed quantity (a whole number of items, 0 or more), ignoring surrounding spaces.
/// Returns e.g. "21 x2 = 42", or None if the input isn't a quantity.
pub fn doubled(input: &str) -> Option<String> {
    let n = input.trim().parse().ok()?;
    Some(format!("{} x2 = {}", n, n * 2))
}

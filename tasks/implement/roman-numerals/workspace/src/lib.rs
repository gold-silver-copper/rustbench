/// Writes `n` as a Roman numeral in standard form, using `I V X L C D M` and
/// the subtractive pairs `IV IX XL XC CD CM` (so 4 is `IV`, never `IIII`,
/// and 1994 is `MCMXCIV`). Returns `None` unless `1 <= n <= 3999`.
///
/// ```
/// assert_eq!(task::to_roman(1994).as_deref(), Some("MCMXCIV"));
/// assert_eq!(task::to_roman(0), None);
/// ```
pub fn to_roman(n: u32) -> Option<String> {
    todo!()
}

/// Reads a Roman numeral written exactly in the standard form that
/// [`to_roman`] produces, uppercase only.
///
/// Returns `None` for anything else: the empty string, lowercase letters,
/// surrounding whitespace, and non-standard forms such as `IIII`, `VV`, `IC`
/// or `MMMM`. In other words, `from_roman(s) == Some(n)` exactly when
/// `to_roman(n)` is `Some(s)`.
///
/// ```
/// assert_eq!(task::from_roman("MCMXCIV"), Some(1994));
/// assert_eq!(task::from_roman("IIII"), None);
/// ```
pub fn from_roman(s: &str) -> Option<u32> {
    todo!()
}

/// Writes `n` as a Roman numeral in standard form, using `I V X L C D M` and
/// the subtractive pairs `IV IX XL XC CD CM` (so 4 is `IV`, never `IIII`,
/// and 1994 is `MCMXCIV`). Returns `None` unless `1 <= n <= 3999`.
///
/// ```
/// assert_eq!(task::to_roman(1994).as_deref(), Some("MCMXCIV"));
/// assert_eq!(task::to_roman(0), None);
/// ```
pub fn to_roman(n: u32) -> Option<String> {
    if !(1..=3999).contains(&n) {
        return None;
    }
    let mut n = n;
    let mut out = String::new();
    for &(v, s) in TABLE {
        while n >= v {
            out.push_str(s);
            n -= v;
        }
    }
    Some(out)
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
    let mut rest = s;
    let mut total = 0u32;
    for &(v, sym) in TABLE {
        while let Some(r) = rest.strip_prefix(sym) {
            rest = r;
            total = total.checked_add(v)?;
        }
    }
    if !rest.is_empty() || to_roman(total).as_deref() != Some(s) {
        return None;
    }
    Some(total)
}

const TABLE: &[(u32, &str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

/// Collapses each run of one repeated character into a single occurrence,
/// but only for characters that appear in `targets`. All other characters,
/// including runs of them, are copied unchanged. Characters are `char`s.
/// An empty `targets` returns `text` unchanged.
///
/// ```
/// assert_eq!(task::squeeze("aa  bb--cc", " -"), "aa bb-cc");
/// ```
pub fn squeeze(text: &str, targets: &str) -> String {
    let mut out = String::new();
    let mut last = None;
    for c in text.chars() {
        if last == Some(c) && targets.contains(c) {
            continue;
        }
        out.push(c);
        last = Some(c);
    }
    out
}

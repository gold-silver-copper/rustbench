/// Collapses each run of one repeated character into a single occurrence,
/// but only for characters that appear in `targets`. All other characters,
/// including runs of them, are copied unchanged. Characters are `char`s.
/// An empty `targets` returns `text` unchanged.
///
/// ```
/// assert_eq!(task::squeeze("aa  bb--cc", " -"), "aa bb-cc");
/// ```
pub fn squeeze(text: &str, targets: &str) -> String {
    todo!()
}

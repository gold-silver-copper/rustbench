/// Checks that the brackets `()`, `[]` and `{}` in `text` are balanced and
/// properly nested. All other characters are ignored.
///
/// Returns `None` when they are. Otherwise returns the position of the first
/// problem, counted in `char`s from 0:
/// - scanning left to right, the first closing bracket that does not match
///   the most recently opened bracket still open (or that comes when no
///   bracket is open);
/// - if there is no such closing bracket, the leftmost opening bracket that
///   is never closed.
///
/// ```
/// assert_eq!(task::first_unbalanced("f(a[1], {b})"), None);
/// assert_eq!(task::first_unbalanced("(]"), Some(1));
/// assert_eq!(task::first_unbalanced("((x)"), Some(0));
/// ```
pub fn first_unbalanced(text: &str) -> Option<usize> {
    todo!()
}

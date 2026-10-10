/// Greedily wraps the words of `text` into lines of at most `width`
/// characters and joins the lines with `\n`.
///
/// - Words are separated by any whitespace; the original spacing and line
///   breaks are not kept.
/// - Words on a line are separated by one space. Each line takes as many of
///   the next words as fit.
/// - Lengths are counted in `char`s.
/// - A word longer than `width` is not split: it goes on a line of its own.
/// - Lines have no leading or trailing spaces and there is no trailing
///   `\n`. Text with no words gives an empty string.
///
/// ```
/// assert_eq!(task::wrap_text("the quick brown fox", 10), "the quick\nbrown fox");
/// ```
pub fn wrap_text(text: &str, width: usize) -> String {
    todo!()
}

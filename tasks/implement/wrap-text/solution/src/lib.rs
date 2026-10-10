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
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    let mut len = 0;
    for word in text.split_whitespace() {
        let n = word.chars().count();
        if len > 0 && len + 1 + n > width {
            lines.push(std::mem::take(&mut line));
            len = 0;
        }
        if len > 0 {
            line.push(' ');
            len += 1;
        }
        line.push_str(word);
        len += n;
    }
    if len > 0 {
        lines.push(line);
    }
    lines.join("\n")
}

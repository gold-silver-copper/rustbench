/// Returns the longest whitespace-separated word in `text`, with length
/// counted in `char`s.
///
/// If several words share the greatest length, the first of them is
/// returned. Returns `None` when `text` has no words.
///
/// ```
/// assert_eq!(task::longest_word("a bb cc d"), Some("bb"));
/// assert_eq!(task::longest_word("   "), None);
/// ```
pub fn longest_word(text: &str) -> Option<&str> {
    let mut best: Option<(&str, usize)> = None;
    for w in text.split_whitespace() {
        let n = w.chars().count();
        if best.is_none_or(|(_, m)| n > m) {
            best = Some((w, n));
        }
    }
    best.map(|(w, _)| w)
}

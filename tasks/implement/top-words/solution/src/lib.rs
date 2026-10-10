/// Returns the `k` most frequent words in `text`, each with its count.
///
/// A word is a maximal run of ASCII letters (`a`-`z`, `A`-`Z`); every other
/// character separates words, so `"don't"` is the two words `don` and `t`.
/// Words are compared case-insensitively and returned in lowercase.
///
/// The result is ordered by count, highest first; words with equal counts
/// are ordered alphabetically. If there are fewer than `k` distinct words,
/// all of them are returned. `k == 0` gives an empty vector.
///
/// ```
/// let got = task::top_words("the cat and the hat. The end", 2);
/// assert_eq!(got, vec![("the".to_string(), 3), ("and".to_string(), 1)]);
/// ```
pub fn top_words(text: &str, k: usize) -> Vec<(String, usize)> {
    let mut counts = std::collections::HashMap::new();
    for w in text.split(|c: char| !c.is_ascii_alphabetic()).filter(|w| !w.is_empty()) {
        *counts.entry(w.to_ascii_lowercase()).or_insert(0usize) += 1;
    }
    let mut all: Vec<(String, usize)> = counts.into_iter().collect();
    all.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    all.truncate(k);
    all
}

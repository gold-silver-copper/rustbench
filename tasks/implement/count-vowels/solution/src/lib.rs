/// Counts the vowels in `text`.
///
/// The vowels are the ASCII letters `a`, `e`, `i`, `o`, `u`, in either case.
/// `y` is not a vowel, and non-ASCII letters (such as `é`) are never counted.
///
/// ```
/// assert_eq!(task::count_vowels("Hello, World"), 3);
/// assert_eq!(task::count_vowels("rhythm"), 0);
/// ```
pub fn count_vowels(text: &str) -> usize {
    text.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

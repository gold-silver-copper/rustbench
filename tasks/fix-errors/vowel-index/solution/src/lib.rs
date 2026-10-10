/// Number of vowels (a, e, i, o, u in either case) in a word.
pub fn vowels(word: &str) -> usize {
    let mut n = 0;
    for c in word.chars() {
        if "aeiouAEIOU".contains(c) {
            n += 1;
        }
    }
    n
}

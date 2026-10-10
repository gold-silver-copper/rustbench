/// Number of vowels (a, e, i, o, u in either case) in a word.
pub fn vowels(word: &str) -> usize {
    let mut n = 0;
    for i in 0..word.len() {
        if "aeiouAEIOU".contains(word[i]) {
            n += 1;
        }
    }
    n
}

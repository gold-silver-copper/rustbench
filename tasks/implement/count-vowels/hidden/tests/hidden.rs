use task::count_vowels;

#[test]
fn both_cases() {
    assert_eq!(count_vowels("AEIOU aeiou"), 10);
    assert_eq!(count_vowels("Programming In Rust"), 5);
}

#[test]
fn not_vowels() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("yYy xyz"), 0);
    assert_eq!(count_vowels("café éèêÉ"), 1);
}

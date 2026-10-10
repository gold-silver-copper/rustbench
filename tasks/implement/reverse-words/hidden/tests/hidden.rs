use task::reverse_words;

#[test]
fn reverses() {
    assert_eq!(reverse_words("hello world"), "world hello");
    assert_eq!(reverse_words("a\n\nb  c\td"), "d c b a");
    assert_eq!(reverse_words("single"), "single");
    assert_eq!(reverse_words("héllo wörld!"), "wörld! héllo");
}

#[test]
fn no_words() {
    assert_eq!(reverse_words(""), "");
    assert_eq!(reverse_words(" \t\n "), "");
}

use task::rle_encode;

#[test]
fn runs() {
    assert_eq!(rle_encode("abc"), "abc");
    assert_eq!(rle_encode("aabaa"), "a2ba2");
    assert_eq!(rle_encode("zzzzzzzzzzzz"), "z12");
    assert_eq!(rle_encode("  x"), " 2x");
    assert_eq!(rle_encode("日日本"), "日2本");
}

#[test]
fn empty() {
    assert_eq!(rle_encode(""), "");
}

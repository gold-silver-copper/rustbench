use task::wrap_text;

#[test]
fn wraps() {
    assert_eq!(wrap_text("aaa bb cc d", 5), "aaa\nbb cc\nd");
    assert_eq!(wrap_text("one two", 7), "one two");
    assert_eq!(wrap_text("one two", 6), "one\ntwo");
    assert_eq!(wrap_text("  lots\n\nof   space  ", 80), "lots of space");
}

#[test]
fn long_words() {
    assert_eq!(wrap_text("a extraordinary b c", 4), "a\nextraordinary\nb c");
    assert_eq!(wrap_text("x y", 0), "x\ny");
}

#[test]
fn counts_chars() {
    assert_eq!(wrap_text("ééé ééé", 7), "ééé ééé");
    assert_eq!(wrap_text("ééé ééé", 6), "ééé\nééé");
}

#[test]
fn empty() {
    assert_eq!(wrap_text("", 10), "");
    assert_eq!(wrap_text(" \n ", 10), "");
}

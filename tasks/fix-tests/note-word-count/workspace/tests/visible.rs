use task::{reading_minutes, word_count};

#[test]
fn empty_note_has_no_words() {
    assert_eq!(word_count(""), 0);
    assert_eq!(reading_minutes(""), 0);
}

use task::{reading_minutes, word_count};

#[test]
fn whitespace_runs() {
    assert_eq!(word_count("   "), 0);
    assert_eq!(word_count("  two   spaces\nand a newline "), 5);
    assert_eq!(word_count("tab\tseparated"), 2);
    assert_eq!(word_count("one"), 1);
}

#[test]
fn reading_time() {
    let text = "word ".repeat(201);
    assert_eq!(reading_minutes(&text), 2);
    assert_eq!(reading_minutes("hi"), 1);
    assert_eq!(reading_minutes("\n\n"), 0);
}

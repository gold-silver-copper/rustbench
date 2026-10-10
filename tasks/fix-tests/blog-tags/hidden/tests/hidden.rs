use task::{normalize_tags, parse_tag_line};

#[test]
fn from_a_line() {
    assert_eq!(parse_tag_line("b, A, , a,b"), vec!["a", "b"]);
    assert_eq!(parse_tag_line(""), Vec::<String>::new());
}

#[test]
fn already_clean() {
    assert_eq!(
        normalize_tags(&["go", "zig", "go", "c", "zig"]),
        vec!["c", "go", "zig"]
    );
}

#[test]
fn counts_tags() {
    assert_eq!(task::tag_count("rust, cli,, notes"), 3);
    assert_eq!(task::tag_count(""), 0);
    assert_eq!(task::tag_count(" , "), 0);
}

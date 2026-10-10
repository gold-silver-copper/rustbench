use task::excerpt;

#[test]
fn quotes_lines_two_to_three() {
    assert_eq!(excerpt("a\nb\nc\nd", 2, 3), "b\nc");
}

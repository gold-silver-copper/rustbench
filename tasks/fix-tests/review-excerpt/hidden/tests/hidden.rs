use task::excerpt;

#[test]
fn single_and_past_end() {
    let src = "fn main() {\n    let x = 1;\n    println!(\"{x}\");\n}";
    assert_eq!(excerpt(src, 1, 1), "fn main() {");
    assert_eq!(excerpt(src, 3, 10), "    println!(\"{x}\");\n}");
    assert_eq!(excerpt(src, 5, 9), "");
}

#[test]
fn bad_ranges() {
    assert_eq!(excerpt("a\nb", 0, 2), "");
    assert_eq!(excerpt("a\nb", 2, 1), "");
}

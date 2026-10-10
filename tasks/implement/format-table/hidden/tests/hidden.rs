use task::format_table;

#[test]
fn aligns() {
    assert_eq!(format_table(&[vec!["a", "bb", "c"], vec!["ddd", "e", "f"]]), "a   | bb | c\nddd | e  | f");
    assert_eq!(format_table(&[vec!["x"]]), "x");
    assert_eq!(format_table(&[vec!["k", "", ""], vec!["key", "v"]]), "k   |   |\nkey | v |");
}

#[test]
fn counts_chars() {
    assert_eq!(format_table(&[vec!["été", "1"], vec!["x", "2"]]), "été | 1\nx   | 2");
}

#[test]
fn empty() {
    assert_eq!(format_table(&[]), "");
    assert_eq!(format_table(&[vec![], vec!["a", "b"]]), "  |\na | b");
}

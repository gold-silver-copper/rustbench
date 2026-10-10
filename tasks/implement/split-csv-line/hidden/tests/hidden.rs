use task::{split_csv_line, CsvError};

fn v(fields: &[&str]) -> Result<Vec<String>, CsvError> {
    Ok(fields.iter().map(|s| s.to_string()).collect())
}

#[test]
fn fields() {
    assert_eq!(split_csv_line(""), v(&[""]));
    assert_eq!(split_csv_line(","), v(&["", ""]));
    assert_eq!(split_csv_line("a, b ,c "), v(&["a", " b ", "c "]));
    assert_eq!(split_csv_line(r#""""#), v(&[""]));
    assert_eq!(split_csv_line(r#""""""#), v(&["\""]));
    assert_eq!(split_csv_line(r#"x,"1,2",y"#), v(&["x", "1,2", "y"]));
    assert_eq!(split_csv_line(r#""é,ü""#), v(&["é,ü"]));
    assert_eq!(split_csv_line(r#""a""b",c"#), v(&["a\"b", "c"]));
}

#[test]
fn errors() {
    assert_eq!(split_csv_line(r#""abc"#), Err(CsvError::Unterminated));
    assert_eq!(split_csv_line(r#"a,"b"""#), Err(CsvError::Unterminated));
    assert_eq!(split_csv_line(r#" "x""#), Err(CsvError::StrayQuote(1)));
    assert_eq!(split_csv_line(r#"é"x"#), Err(CsvError::StrayQuote(2)));
    assert_eq!(split_csv_line(r#""a"b"#), Err(CsvError::AfterQuote(3)));
    assert_eq!(split_csv_line(r#""a" ,b"#), Err(CsvError::AfterQuote(3)));
    assert_eq!(split_csv_line(r#"x"y,"z"#), Err(CsvError::StrayQuote(1)));
}

/// Why [`split_csv_line`] rejected a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsvError {
    /// A quoted field is never closed.
    Unterminated,
    /// A `"` at this byte offset is inside an unquoted field.
    StrayQuote(usize),
    /// The character at this byte offset follows a quoted field's closing
    /// quote but is not `,`.
    AfterQuote(usize),
}

/// Splits one line of comma-separated values into its fields.
///
/// - Fields are separated by `,`. An empty line is one empty field, and
///   `a,` is two fields, `a` and an empty one.
/// - A field whose first character is `"` is quoted. It ends at the next
///   `"` that is not doubled; inside it, `""` stands for one `"` and commas
///   are ordinary characters. The closing quote must be followed by `,` or
///   the end of the line.
/// - Any other field is unquoted and taken exactly as written, spaces
///   included. It must not contain `"`.
///
/// Errors are reported for the first problem found reading left to right.
///
/// ```
/// assert_eq!(
///     task::split_csv_line(r#"a,"b,c","say ""hi""",,"#),
///     Ok(vec!["a".to_string(), "b,c".to_string(), "say \"hi\"".to_string(), String::new(), String::new()])
/// );
/// assert_eq!(task::split_csv_line(r#"ab"c"#), Err(task::CsvError::StrayQuote(2)));
/// ```
pub fn split_csv_line(line: &str) -> Result<Vec<String>, CsvError> {
    todo!()
}

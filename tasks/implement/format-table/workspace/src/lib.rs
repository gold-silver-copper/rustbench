/// Lays out `rows` of cells as an aligned text table.
///
/// - Rows may have different numbers of cells; a short row is treated as if
///   it had empty cells at the end.
/// - Each column is as wide as its widest cell, measured in `char`s.
/// - Each cell is padded on the right with spaces to its column's width,
///   and the cells of a row are joined with `" | "`.
/// - Then trailing spaces are removed from the end of each line.
/// - Lines are joined with `\n`, with no trailing newline. No rows gives an
///   empty string.
///
/// ```
/// let t = task::format_table(&[vec!["name", "qty"], vec!["apple", "3"], vec!["fig"]]);
/// assert_eq!(t, "name  | qty\napple | 3\nfig   |");
/// ```
pub fn format_table(rows: &[Vec<&str>]) -> String {
    todo!()
}

//! Quoting lines from a source file in code-review comments.

/// Lines `first..=last` of `text` (1-based, inclusive), joined with newlines.
/// Line numbers past the end are ignored.
pub fn excerpt(text: &str, first: usize, last: usize) -> String {
    if first == 0 || last < first {
        return String::new();
    }
    text.lines()
        .skip(first)
        .take(last - first + 1)
        .collect::<Vec<_>>()
        .join("\n")
}

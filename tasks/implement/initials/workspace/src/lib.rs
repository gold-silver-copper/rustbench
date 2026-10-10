/// Returns the initials of `name`: the first character of each
/// whitespace-separated word, uppercased, concatenated with no separator.
///
/// A word whose first character is not alphabetic (`char::is_alphabetic`) is
/// skipped. Uppercasing uses `char::to_uppercase`, which can give more than
/// one character (`ß` becomes `SS`). No words gives an empty string.
///
/// ```
/// assert_eq!(task::initials("ada  lovelace"), "AL");
/// assert_eq!(task::initials("jo 'jay' o'neil 3rd"), "JO");
/// ```
pub fn initials(name: &str) -> String {
    todo!()
}

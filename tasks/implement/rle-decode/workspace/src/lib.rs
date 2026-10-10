/// Decodes run-length text in which each character is followed by an
/// optional decimal count, so `"a3bc2"` decodes to `"aaabcc"`.
///
/// The input is a sequence of items. Each item is one character that is not
/// an ASCII digit, followed by an optional count: a run of ASCII digits
/// giving how many times to repeat the character. No count means once.
/// Characters are `char`s, so `"é2"` decodes to `"éé"`. A count must be in
/// `1..=65535` and must not have leading zeros. An empty input decodes to an
/// empty string.
///
/// Returns `Err(offset)`, a byte offset into `encoded`, when the input is
/// invalid:
/// - it starts with a digit: offset 0;
/// - a count is 0, has a leading zero, or is above 65535: the offset of the
///   count's first digit.
///
/// ```
/// assert_eq!(task::rle_decode("a3bc2"), Ok("aaabcc".to_string()));
/// assert_eq!(task::rle_decode("a0"), Err(1));
/// ```
pub fn rle_decode(encoded: &str) -> Result<String, usize> {
    todo!()
}

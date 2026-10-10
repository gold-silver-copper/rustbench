/// Run-length encodes `text`.
///
/// Each maximal run of one repeated character becomes that character
/// followed by the run's length in decimal, except that the length is left
/// out when it is 1. Characters are `char`s (Unicode scalar values), not
/// bytes. An empty string encodes to an empty string.
///
/// ```
/// assert_eq!(task::rle_encode("aaabcc"), "a3bc2");
/// assert_eq!(task::rle_encode("ééé"), "é3");
/// ```
pub fn rle_encode(text: &str) -> String {
    todo!()
}

/// Shifts every ASCII letter in `text` by `shift` places in the alphabet,
/// wrapping around (`z` shifted by 1 is `a`) and keeping its case.
///
/// `shift` may be negative (shifting backwards) and may be any size, in
/// either direction. Every other character, non-ASCII letters included, is
/// unchanged.
///
/// ```
/// assert_eq!(task::caesar_shift("Hello, World!", 3), "Khoor, Zruog!");
/// assert_eq!(task::caesar_shift("abc", -1), "zab");
/// ```
pub fn caesar_shift(text: &str, shift: i32) -> String {
    let k = shift.rem_euclid(26) as u8;
    text.chars()
        .map(|c| {
            let base = match c {
                'a'..='z' => b'a',
                'A'..='Z' => b'A',
                _ => return c,
            };
            char::from(base + (c as u8 - base + k) % 26)
        })
        .collect()
}

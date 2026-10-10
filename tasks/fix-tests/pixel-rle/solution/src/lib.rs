//! Run-length encoding for a pixel-art editor's save format: "aaab" -> "3a1b".
//! Pixels are any non-digit characters.

pub fn encode(row: &str) -> String {
    let mut out = String::new();
    let mut current: Option<char> = None;
    let mut count = 0;
    for c in row.chars() {
        match current {
            Some(p) if p == c => count += 1,
            Some(p) => {
                out.push_str(&format!("{count}{p}"));
                current = Some(c);
                count = 1;
            }
            None => {
                current = Some(c);
                count = 1;
            }
        }
    }
    if let Some(p) = current {
        out.push_str(&format!("{count}{p}"));
    }
    out
}

/// The row back from its encoding, or `None` if the encoding is malformed.
pub fn decode(encoded: &str) -> Option<String> {
    let mut row = String::new();
    let mut count = 0usize;
    for c in encoded.chars() {
        if let Some(d) = c.to_digit(10) {
            count = count * 10 + d as usize;
        } else {
            if count == 0 {
                return None;
            }
            row.extend(std::iter::repeat_n(c, count));
            count = 0;
        }
    }
    (count == 0).then_some(row)
}

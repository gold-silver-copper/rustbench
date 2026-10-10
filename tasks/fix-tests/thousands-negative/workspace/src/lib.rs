//! Formatting numbers for a finance dashboard: 1234567 -> "1,234,567".

/// `n` with a comma between every group of three digits.
pub fn group_thousands(n: i64) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

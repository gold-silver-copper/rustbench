use crate::media::{Book, Describe};

/// One numbered line per item.
pub fn catalog(items: &[Book]) -> String {
    let mut out = String::new();
    for (i, item) in items.iter().enumerate() {
        out.push_str(&format!("{}. {}\n", i + 1, item.describe()));
    }
    out
}

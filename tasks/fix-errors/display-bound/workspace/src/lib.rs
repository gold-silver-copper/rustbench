/// Renders items as a numbered list: "1. apples\n2. pears\n".
pub fn numbered<T>(items: &[T]) -> String {
    let mut out = String::new();
    for (i, item) in items.iter().enumerate() {
        out.push_str(&format!("{}. {}\n", i + 1, item));
    }
    out
}

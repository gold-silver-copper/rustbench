/// Name in roster order: "Last, First".
pub fn roster_name(first: &str, last: &str) -> String {
    last.to_string() + ", " + first
}

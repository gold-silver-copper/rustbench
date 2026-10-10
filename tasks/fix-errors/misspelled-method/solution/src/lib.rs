/// How many names on the waiting list are longer than `limit` bytes.
pub fn long_names(names: &[String], limit: usize) -> usize {
    names.iter().filter(|n| n.len() > limit).count()
}

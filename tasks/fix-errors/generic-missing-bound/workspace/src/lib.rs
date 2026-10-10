/// The largest item, or None for an empty slice.
pub fn largest<T: Copy>(items: &[T]) -> Option<T> {
    let mut best = *items.first()?;
    for &item in items {
        if item > best {
            best = item;
        }
    }
    Some(best)
}

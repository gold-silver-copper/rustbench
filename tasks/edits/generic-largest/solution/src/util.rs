/// The largest item, or `None` for an empty slice.
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    let mut best = iter.next()?;
    for x in iter {
        if x > best {
            best = x;
        }
    }
    Some(best)
}

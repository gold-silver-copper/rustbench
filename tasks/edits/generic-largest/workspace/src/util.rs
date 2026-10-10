/// The largest item, or `None` for an empty slice.
pub fn largest(items: &[i32]) -> Option<i32> {
    let mut iter = items.iter().copied();
    let mut best = iter.next()?;
    for x in iter {
        if x > best {
            best = x;
        }
    }
    Some(best)
}

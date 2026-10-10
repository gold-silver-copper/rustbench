/// The items on page `page` (counting from 0), ten per page.
/// Pages past the end are empty.
pub fn paginate<T: Clone>(items: &[T], page: usize) -> Vec<T> {
    let start = page.saturating_mul(10);
    items.iter().skip(start).take(10).cloned().collect()
}

/// How many pages of ten `len` items fill.
pub fn page_count(len: usize) -> usize {
    len.div_ceil(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_page() {
        let v: Vec<u32> = (0..25).collect();
        assert_eq!(paginate(&v, 1), (10..20).collect::<Vec<_>>());
        assert_eq!(paginate(&v, 2), vec![20, 21, 22, 23, 24]);
        assert!(paginate(&v, 3).is_empty());
    }
}

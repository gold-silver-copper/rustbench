/// The items on page `page` (counting from 0), `page_size` per page.
/// Pages past the end are empty.
pub fn paginate<T: Clone>(items: &[T], page: usize, page_size: usize) -> Vec<T> {
    let start = page.saturating_mul(page_size);
    items.iter().skip(start).take(page_size).cloned().collect()
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
        assert_eq!(paginate(&v, 1, 10), (10..20).collect::<Vec<_>>());
        assert_eq!(paginate(&v, 2, 10), vec![20, 21, 22, 23, 24]);
        assert!(paginate(&v, 3, 10).is_empty());
    }
}

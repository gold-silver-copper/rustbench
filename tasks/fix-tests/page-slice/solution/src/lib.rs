//! Paging through search results.

/// Items on page `page` (0-based) when showing `per_page` items per page.
/// Pages past the end are empty; the last page may be short.
pub fn page<T: Clone>(items: &[T], page: usize, per_page: usize) -> Vec<T> {
    let start = page * per_page;
    if start >= items.len() {
        return Vec::new();
    }
    let end = (start + per_page).min(items.len());
    items[start..end].to_vec()
}

/// Number of pages needed to show every item.
pub fn page_count(total: usize, per_page: usize) -> usize {
    if per_page == 0 {
        return 0;
    }
    total.div_ceil(per_page)
}

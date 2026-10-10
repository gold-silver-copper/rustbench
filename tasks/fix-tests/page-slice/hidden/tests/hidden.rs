use task::{page, page_count};

#[test]
fn middle_and_last_pages() {
    let results: Vec<u32> = (1..=7).collect();
    assert_eq!(page(&results, 1, 3), vec![4, 5, 6]);
    assert_eq!(page(&results, 2, 3), vec![7]);
    assert_eq!(page(&results, 3, 3), Vec::<u32>::new());
    assert_eq!(page(&results, 0, 10), results);
}

#[test]
fn counts_pages() {
    assert_eq!(page_count(7, 3), 3);
    assert_eq!(page_count(6, 3), 2);
    assert_eq!(page_count(0, 3), 0);
}

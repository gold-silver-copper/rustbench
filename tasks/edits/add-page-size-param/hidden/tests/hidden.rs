use task::listing::{Contact, page_lines, search};
use task::paginate;

#[test]
fn custom_page_sizes() {
    let v: Vec<u32> = (0..10).collect();
    assert_eq!(paginate(&v, 0, 3), vec![0, 1, 2]);
    assert_eq!(paginate(&v, 3, 3), vec![9]);
    assert!(paginate(&v, 4, 3).is_empty());
    assert_eq!(paginate(&v, 1, 4), vec![4, 5, 6, 7]);
    assert_eq!(paginate(&v, 0, 10), v);
}

#[test]
fn listing_still_uses_ten_per_page() {
    let contacts: Vec<Contact> = (0..23)
        .map(|i| Contact { name: format!("n{i:02}"), phone: String::new() })
        .collect();
    assert_eq!(page_lines(&contacts, 0).len(), 10);
    assert_eq!(page_lines(&contacts, 2).len(), 3);
    assert_eq!(search(&contacts, "n").len(), 10);
    assert_eq!(search(&contacts, "n2").len(), 3);
}

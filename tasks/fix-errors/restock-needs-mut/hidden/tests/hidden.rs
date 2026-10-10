#[test]
fn restocks() {
    let mut s = task::Shelf { counts: vec![2, 10, 0], capacity: 10 };
    assert_eq!(s.restock_all(), 18);
    assert_eq!(s.counts, vec![10, 10, 10]);
    assert_eq!(s.restock(5), 0);
}

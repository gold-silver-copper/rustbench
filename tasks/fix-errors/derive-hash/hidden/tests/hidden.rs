use task::Cell;

#[test]
fn counts_hits() {
    let a = Cell { row: 1, col: 2 };
    let b = Cell { row: 0, col: 0 };
    let c = task::hit_counts(&[a.clone(), b.clone(), a.clone()]);
    assert_eq!(c.get(&a), Some(&2));
    assert_eq!(c.get(&b), Some(&1));
    assert_eq!(c.len(), 2);
}

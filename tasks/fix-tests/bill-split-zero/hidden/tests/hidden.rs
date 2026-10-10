use task::split;

#[test]
fn shares() {
    assert_eq!(split(1000, 3), vec![334, 333, 333]);
    assert_eq!(split(1001, 2), vec![501, 500]);
    assert_eq!(split(0, 2), vec![0, 0]);
    assert_eq!(split(7, 1), vec![7]);
}

#[test]
fn zero_people() {
    assert_eq!(split(0, 0), Vec::<u64>::new());
    assert_eq!(split(55, 0), Vec::<u64>::new());
}

use task::compress_ranges;

#[test]
fn runs() {
    assert_eq!(compress_ranges(&[1, 2]), "1,2");
    assert_eq!(compress_ranges(&[1, 2, 3]), "1-3");
    assert_eq!(compress_ranges(&[10, 4, 6, 5, 11, 7, 20]), "4-7,10,11,20");
    assert_eq!(compress_ranges(&[3, 3, 3]), "3");
    assert_eq!(compress_ranges(&[-1, 0, 1, -5]), "-5,-1-1");
}

#[test]
fn extremes() {
    assert_eq!(compress_ranges(&[i32::MAX, i32::MAX - 1, i32::MAX - 2]), "2147483645-2147483647");
    assert_eq!(compress_ranges(&[i32::MIN, i32::MAX]), "-2147483648,2147483647");
    assert_eq!(compress_ranges(&[i32::MIN + 1, i32::MIN]), "-2147483648,-2147483647");
}

#[test]
fn empty() {
    assert_eq!(compress_ranges(&[]), "");
}

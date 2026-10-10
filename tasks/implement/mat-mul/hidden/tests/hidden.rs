use task::{mat_mul, MatError};

#[test]
fn multiplies() {
    let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];
    assert_eq!(mat_mul(&a, &b), Ok(vec![vec![58, 64], vec![139, 154]]));
    assert_eq!(mat_mul(&[vec![2]], &[vec![-3]]), Ok(vec![vec![-6]]));
    let col = vec![vec![1], vec![2]];
    let row = vec![vec![3, 4]];
    assert_eq!(mat_mul(&col, &row), Ok(vec![vec![3, 4], vec![6, 8]]));
    assert_eq!(mat_mul(&row, &col), Ok(vec![vec![11]]));
}

#[test]
fn shape_errors() {
    let ok = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(mat_mul(&[], &ok), Err(MatError::Empty));
    assert_eq!(mat_mul(&ok, &[]), Err(MatError::Empty));
    assert_eq!(mat_mul(&[vec![]], &ok), Err(MatError::Empty));
    assert_eq!(mat_mul(&[vec![1, 2], vec![3]], &ok), Err(MatError::Ragged));
    assert_eq!(mat_mul(&ok, &[vec![1], vec![2, 3]]), Err(MatError::Ragged));
    assert_eq!(
        mat_mul(&ok, &[vec![1], vec![2], vec![3]]),
        Err(MatError::Mismatch { left_cols: 2, right_rows: 3 })
    );
}

#[test]
fn error_order() {
    // Empty is checked before Ragged, Ragged before Mismatch.
    assert_eq!(mat_mul(&[vec![1], vec![2, 3]], &[]), Err(MatError::Empty));
    assert_eq!(mat_mul(&[vec![1, 2, 3]], &[vec![1], vec![2, 3]]), Err(MatError::Ragged));
}

#[test]
fn overflow() {
    assert_eq!(mat_mul(&[vec![i64::MAX]], &[vec![2]]), Err(MatError::Overflow));
    assert_eq!(mat_mul(&[vec![i64::MAX, 1]], &[vec![1], vec![1]]), Err(MatError::Overflow));
    assert_eq!(mat_mul(&[vec![i64::MAX, -1]], &[vec![1], vec![1]]), Ok(vec![vec![i64::MAX - 1]]));
}

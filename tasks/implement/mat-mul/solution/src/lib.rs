/// Why [`mat_mul`] could not multiply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatError {
    /// A matrix has no rows, or its first row is empty.
    Empty,
    /// A row of a matrix has a different length from that matrix's first row.
    Ragged,
    /// The left matrix's column count differs from the right one's row count.
    Mismatch { left_cols: usize, right_rows: usize },
    /// An intermediate or final value does not fit in `i64`.
    Overflow,
}

/// Multiplies the matrices `a` (`n × m`) and `b` (`m × p`), each given as a
/// list of rows, giving the `n × p` matrix with
/// `result[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + ...`.
///
/// The checks run in this order, and the first that fails is returned:
/// 1. `Empty` if either matrix has no rows or has an empty first row;
/// 2. `Ragged` if any row of either matrix differs in length from that
///    matrix's first row;
/// 3. `Mismatch` if `a`'s column count is not `b`'s row count;
/// 4. `Overflow` if, while summing the products for an entry from left to
///    right with `i64` arithmetic, any product or partial sum overflows.
///
/// ```
/// let a = vec![vec![1, 2], vec![3, 4]];
/// let b = vec![vec![5], vec![6]];
/// assert_eq!(task::mat_mul(&a, &b), Ok(vec![vec![17], vec![39]]));
/// assert_eq!(
///     task::mat_mul(&b, &b),
///     Err(task::MatError::Mismatch { left_cols: 1, right_rows: 2 })
/// );
/// ```
pub fn mat_mul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Result<Vec<Vec<i64>>, MatError> {
    fn cols(m: &[Vec<i64>]) -> Result<usize, MatError> {
        match m.first() {
            Some(r) if !r.is_empty() => Ok(r.len()),
            _ => Err(MatError::Empty),
        }
    }
    let (m, p) = (cols(a)?, cols(b)?);
    if a.iter().any(|r| r.len() != m) || b.iter().any(|r| r.len() != p) {
        return Err(MatError::Ragged);
    }
    if m != b.len() {
        return Err(MatError::Mismatch { left_cols: m, right_rows: b.len() });
    }
    a.iter()
        .map(|row| {
            (0..p)
                .map(|j| {
                    row.iter().zip(b).try_fold(0i64, |acc, (x, brow)| {
                        let y = brow.get(j).copied().unwrap_or_default();
                        x.checked_mul(y).and_then(|v| acc.checked_add(v))
                    })
                    .ok_or(MatError::Overflow)
                })
                .collect()
        })
        .collect()
}

/// Merges closed integer intervals.
///
/// Each `(start, end)` stands for the integers `start..=end`. Returns their
/// union as the shortest list of disjoint intervals, sorted by start:
/// intervals that overlap *or are adjacent* (one ends at `k`, another starts
/// at `k + 1`) become one. The input may be in any order and may contain
/// duplicates; an empty input gives an empty vector.
///
/// Returns `Err(i)` if the interval at index `i` has `start > end` (the
/// smallest such `i`). Must work across the whole `i64` range, including
/// `i64::MIN` and `i64::MAX`.
///
/// ```
/// assert_eq!(
///     task::merge_intervals(&[(5, 8), (1, 3), (4, 4), (10, 12), (11, 11)]),
///     Ok(vec![(1, 8), (10, 12)])
/// );
/// assert_eq!(task::merge_intervals(&[(1, 2), (3, 0)]), Err(1));
/// ```
pub fn merge_intervals(intervals: &[(i64, i64)]) -> Result<Vec<(i64, i64)>, usize> {
    if let Some(i) = intervals.iter().position(|(s, e)| s > e) {
        return Err(i);
    }
    let mut sorted = intervals.to_vec();
    sorted.sort_unstable();
    let mut out: Vec<(i64, i64)> = Vec::new();
    for (s, e) in sorted {
        match out.last_mut() {
            Some(last) if last.1 == i64::MAX || s <= last.1 + 1 => last.1 = last.1.max(e),
            _ => out.push((s, e)),
        }
    }
    Ok(out)
}

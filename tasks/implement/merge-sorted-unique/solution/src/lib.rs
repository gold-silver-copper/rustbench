/// Merges two slices, each sorted in ascending order, into one strictly
/// ascending vector.
///
/// Every duplicate is removed, whether it repeats within one slice or
/// appears in both. Either slice may be empty.
///
/// ```
/// assert_eq!(task::merge_sorted_unique(&[1, 3, 3, 7], &[2, 3, 8]), vec![1, 2, 3, 7, 8]);
/// ```
pub fn merge_sorted_unique(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (a.iter().peekable(), b.iter().peekable());
    loop {
        let next = match (i.peek(), j.peek()) {
            (Some(x), Some(y)) if x <= y => i.next(),
            (Some(_), Some(_)) => j.next(),
            (Some(_), None) => i.next(),
            (None, _) => j.next(),
        };
        let Some(&v) = next else { break };
        if out.last() != Some(&v) {
            out.push(v);
        }
    }
    out
}

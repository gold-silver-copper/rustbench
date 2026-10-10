/// Describes a set of integers compactly.
///
/// The values are sorted ascending and duplicates are dropped. Each maximal
/// run of consecutive integers with three or more members is written
/// `start-end`; members of shorter runs are written one by one. The parts
/// are joined with `,` (no spaces). Negative numbers keep their sign, so the
/// run from -3 to -1 is `-3--1`. An empty slice gives an empty string.
///
/// ```
/// assert_eq!(task::compress_ranges(&[5, 1, 2, 3, 3, 8, 9]), "1-3,5,8,9");
/// assert_eq!(task::compress_ranges(&[-3, -2, -1, 4]), "-3--1,4");
/// ```
pub fn compress_ranges(values: &[i32]) -> String {
    todo!()
}

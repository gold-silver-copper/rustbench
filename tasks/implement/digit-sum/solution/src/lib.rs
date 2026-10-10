/// Returns the sum of the decimal digits of `n`. `digit_sum(0)` is 0.
///
/// ```
/// assert_eq!(task::digit_sum(9045), 18);
/// ```
pub fn digit_sum(n: u64) -> u32 {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10) as u32;
        n /= 10;
    }
    sum
}

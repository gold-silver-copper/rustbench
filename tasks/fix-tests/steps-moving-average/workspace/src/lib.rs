//! Smoothing daily step counts with a moving average.

/// Average of every window of `k` consecutive days, in order.
/// Returns nothing when `k` is 0 or longer than the data.
pub fn moving_average(days: &[u32], k: usize) -> Vec<f64> {
    if k == 0 || k > days.len() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for start in 0..days.len() - k {
        let sum: u32 = days[start..start + k].iter().sum();
        out.push(f64::from(sum) / k as f64);
    }
    out
}

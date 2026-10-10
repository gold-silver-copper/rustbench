//! Backoff schedule for retrying a failed upload.

/// Delays in milliseconds before each retry: `base_ms`, then doubling, one per
/// retry allowed, each capped at `max_ms`.
pub fn delays(retries: u32, base_ms: u64, max_ms: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut delay = base_ms;
    for _ in 1..retries {
        out.push(delay.min(max_ms));
        delay = delay.saturating_mul(2);
    }
    out
}

/// The longest an upload can spend waiting between attempts.
pub fn total_wait_ms(retries: u32, base_ms: u64, max_ms: u64) -> u64 {
    delays(retries, base_ms, max_ms).iter().sum()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub name: String,
    pub priority: u8,
    pub runs_at: u32,
}

/// Jobs due at `now`, highest priority first, ties by name.
pub fn due(jobs: &[Job], now: u32) -> Vec<&Job> {
    let mut v: Vec<&Job> = jobs.iter().filter(|j| j.runs_at <= now).collect();
    v.sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| a.name.cmp(&b.name)));
    v
}

/// Delay before retrying a failed job, in seconds.
pub fn retry_delay(attempt: u32) -> u32 {
    30 * 2u32.saturating_pow(attempt.min(6))
}

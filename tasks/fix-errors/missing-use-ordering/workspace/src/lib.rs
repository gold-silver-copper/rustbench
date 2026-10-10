pub struct Runner {
    pub name: String,
    pub seconds: f64,
}

/// Sorts runners fastest first; ties keep their original order.
pub fn rank(runners: &mut [Runner]) {
    runners.sort_by(|a, b| a.seconds.partial_cmp(&b.seconds).unwrap_or(Ordering::Equal));
}

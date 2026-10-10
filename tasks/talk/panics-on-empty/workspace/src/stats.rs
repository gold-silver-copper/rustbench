pub fn mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

pub fn median(xs: &[f64]) -> f64 {
    let mut v = xs.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    let mid = v.len() / 2;
    if v.len() % 2 == 0 {
        (v[mid - 1] + v[mid]) / 2.0
    } else {
        v[mid]
    }
}

pub fn spread(xs: &[f64]) -> Option<f64> {
    let max = xs.iter().copied().reduce(f64::max)?;
    let min = xs.iter().copied().reduce(f64::min)?;
    Some(max - min)
}

pub fn total_count(counts: &[u32]) -> u32 {
    counts.iter().sum()
}

pub fn first_or_zero(xs: &[f64]) -> f64 {
    xs.first().copied().unwrap_or(0.0)
}

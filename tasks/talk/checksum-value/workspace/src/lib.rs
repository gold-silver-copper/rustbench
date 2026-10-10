fn digit_sum(n: u64) -> u64 {
    n.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(u64::from)
        .sum()
}

/// Repeated digit sum until one digit is left.
pub fn digital_root(mut n: u64) -> u64 {
    while n >= 10 {
        n = digit_sum(n);
    }
    n
}

/// Batch checksum: each id's digital root weighted by its 1-based
/// position, summed, last digit.
pub fn checksum(ids: &[u64]) -> u64 {
    ids.iter()
        .enumerate()
        .map(|(i, &id)| digital_root(id) * (i as u64 + 1))
        .sum::<u64>()
        % 10
}

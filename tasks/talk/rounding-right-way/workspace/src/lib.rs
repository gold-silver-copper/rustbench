/// Tax on a line, in cents, rounded half away from zero.
pub fn line_tax_cents(net_cents: i64, rate_percent: f64) -> i64 {
    (net_cents as f64 * rate_percent / 100.0).round() as i64
}

/// Invoice tax: each line rounded, then summed.
pub fn invoice_tax_cents(lines: &[i64], rate_percent: f64) -> i64 {
    lines.iter().map(|&n| line_tax_cents(n, rate_percent)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_half_up() {
        assert_eq!(line_tax_cents(250, 10.0), 25);
        assert_eq!(line_tax_cents(5, 10.0), 1);
    }
}

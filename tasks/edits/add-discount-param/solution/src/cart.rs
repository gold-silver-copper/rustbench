#[derive(Debug, Clone)]
pub struct LineItem {
    pub name: String,
    /// Price of one unit, in cents.
    pub unit_cents: u64,
    pub qty: u32,
}

impl LineItem {
    pub fn new(name: &str, unit_cents: u64, qty: u32) -> Self {
        LineItem {
            name: name.to_string(),
            unit_cents,
            qty,
        }
    }

    pub fn subtotal(&self) -> u64 {
        self.unit_cents * u64::from(self.qty)
    }
}

/// Sum of all line items minus `discount_percent` percent, in cents
/// (rounded down).
pub fn total(items: &[LineItem], discount_percent: u8) -> u64 {
    let sum: u64 = items.iter().map(LineItem::subtotal).sum();
    let keep = 100 - u64::from(discount_percent.min(100));
    sum * keep / 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_lines() {
        let items = [LineItem::new("pen", 150, 4), LineItem::new("pad", 325, 1)];
        assert_eq!(total(&items, 0), 925);
    }

    #[test]
    fn empty_is_zero() {
        assert_eq!(total(&[], 0), 0);
    }
}

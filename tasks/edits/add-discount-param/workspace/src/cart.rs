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

/// Sum of all line items, in cents.
pub fn total(items: &[LineItem]) -> u64 {
    items.iter().map(LineItem::subtotal).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_lines() {
        let items = [LineItem::new("pen", 150, 4), LineItem::new("pad", 325, 1)];
        assert_eq!(total(&items), 925);
    }

    #[test]
    fn empty_is_zero() {
        assert_eq!(total(&[]), 0);
    }
}

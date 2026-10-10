use crate::currency::Currency;

#[derive(Debug, Clone, Copy)]
pub struct Money {
    pub cents: i64,
    pub currency: Currency,
}

impl Money {
    pub fn new(cents: i64, currency: Currency) -> Money {
        Money { cents, currency }
    }

    /// Sum of two amounts, or `None` if the currencies differ.
    pub fn checked_add(self, other: Money) -> Option<Money> {
        if self.currency.code() != other.currency.code() {
            return None;
        }
        Some(Money::new(self.cents + other.cents, self.currency))
    }

    /// "12.50 EUR"
    pub fn display(self) -> String {
        let sign = if self.cents < 0 { "-" } else { "" };
        let c = self.cents.unsigned_abs();
        format!("{sign}{}.{:02} {}", c / 100, c % 100, self.currency.code())
    }
}

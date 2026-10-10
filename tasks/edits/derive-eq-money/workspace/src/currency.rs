#[derive(Debug, Clone, Copy)]
pub enum Currency {
    Eur,
    Usd,
    Gbp,
}

impl Currency {
    pub fn code(self) -> &'static str {
        match self {
            Currency::Eur => "EUR",
            Currency::Usd => "USD",
            Currency::Gbp => "GBP",
        }
    }
}

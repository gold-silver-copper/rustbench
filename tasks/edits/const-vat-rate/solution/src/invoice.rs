use crate::pricing::VAT_RATE;

pub struct Line {
    pub item: String,
    pub qty: u32,
    pub unit_cents: u64,
}

pub struct Invoice {
    pub lines: Vec<Line>,
}

impl Invoice {
    pub fn net_total(&self) -> u64 {
        self.lines
            .iter()
            .map(|l| l.qty as u64 * l.unit_cents)
            .sum()
    }

    pub fn vat_total(&self) -> u64 {
        (self.net_total() as f64 * VAT_RATE).round() as u64
    }

    pub fn gross_total(&self) -> u64 {
        self.net_total() + self.vat_total()
    }

    pub fn footer(&self) -> String {
        format!(
            "VAT {:.0}%: {} cents, total {} cents",
            VAT_RATE * 100.0,
            self.vat_total(),
            self.gross_total()
        )
    }
}

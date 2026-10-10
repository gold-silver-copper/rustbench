/// Standard VAT rate.
pub const STANDARD_RATE: f64 = 0.20;
/// Reduced VAT rate for food and books.
pub const REDUCED_RATE: f64 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
    General,
    Food,
    Books,
}

pub fn rate_for(c: Category) -> f64 {
    match c {
        Category::General => STANDARD_RATE,
        Category::Food | Category::Books => REDUCED_RATE,
    }
}

pub fn gross_cents(net_cents: u64, c: Category) -> u64 {
    (net_cents as f64 * (1.0 + rate_for(c))).round() as u64
}

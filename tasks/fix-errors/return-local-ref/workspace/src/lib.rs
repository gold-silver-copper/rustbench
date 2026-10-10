pub struct Product {
    pub name: String,
    pub price_cents: u32,
}

impl Product {
    /// Shelf label such as "Tea - $3.50".
    pub fn label(&self) -> &str {
        let text = format!("{} - ${}.{:02}", self.name, self.price_cents / 100, self.price_cents % 100);
        &text
    }
}

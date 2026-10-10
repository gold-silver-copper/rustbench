#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub cents: u32,
    pub qty: u32,
}

#[derive(Debug, Default)]
pub struct Cart {
    pub items: Vec<Item>,
    /// Sum of item prices, kept up to date by `add`.
    pub amount: u32,
}

impl Cart {
    pub fn add(&mut self, item: Item) {
        self.amount += item.cents * item.qty;
        self.items.push(item);
    }
}

/// Cart value after a percentage discount.
pub fn cart_value(cart: &Cart, discount_percent: u32) -> u32 {
    cart.amount - cart.amount * discount_percent.min(100) / 100
}

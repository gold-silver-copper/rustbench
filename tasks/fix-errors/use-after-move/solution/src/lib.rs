pub struct Order {
    pub customer: String,
    pub items: Vec<String>,
}

/// Builds an order and the confirmation line shown to the customer.
pub fn place_order(customer: String, items: Vec<String>) -> (Order, String) {
    let order = Order { customer, items };
    let line = format!("Order for {} with {} item(s)", order.customer, order.items.len());
    (order, line)
}

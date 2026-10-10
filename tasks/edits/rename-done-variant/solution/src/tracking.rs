use crate::order::Order;
use crate::status::OrderStatus;

/// Number of orders that reached the customer.
pub fn delivered_count(orders: &[Order]) -> usize {
    orders.iter().filter(|o| o.status == OrderStatus::Delivered).count()
}

/// Orders that still need work from us.
pub fn open_orders(orders: &[Order]) -> Vec<u32> {
    orders
        .iter()
        .filter(|o| !matches!(o.status, OrderStatus::Delivered | OrderStatus::Cancelled))
        .map(|o| o.id)
        .collect()
}

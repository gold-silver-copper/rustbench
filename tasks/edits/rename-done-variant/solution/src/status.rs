#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Placed,
    Packed,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    /// The status after this one, if the order can still move forward.
    pub fn next(self) -> Option<OrderStatus> {
        match self {
            OrderStatus::Placed => Some(OrderStatus::Packed),
            OrderStatus::Packed => Some(OrderStatus::Shipped),
            OrderStatus::Shipped => Some(OrderStatus::Delivered),
            OrderStatus::Delivered | OrderStatus::Cancelled => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            OrderStatus::Placed => "placed",
            OrderStatus::Packed => "packed",
            OrderStatus::Shipped => "on its way",
            OrderStatus::Delivered => "delivered",
            OrderStatus::Cancelled => "cancelled",
        }
    }
}

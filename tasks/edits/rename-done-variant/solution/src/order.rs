use crate::status::OrderStatus;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u32,
    pub status: OrderStatus,
}

impl Order {
    pub fn new(id: u32) -> Order {
        Order { id, status: OrderStatus::Placed }
    }

    /// Move to the next status; returns false if the order is finished.
    pub fn advance(&mut self) -> bool {
        match self.status.next() {
            Some(s) => {
                self.status = s;
                true
            }
            None => false,
        }
    }

    /// Cancelling is only possible before shipping.
    pub fn cancel(&mut self) -> bool {
        match self.status {
            OrderStatus::Placed | OrderStatus::Packed => {
                self.status = OrderStatus::Cancelled;
                true
            }
            OrderStatus::Shipped | OrderStatus::Delivered | OrderStatus::Cancelled => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_to_the_end() {
        let mut o = Order::new(1);
        while o.advance() {}
        assert_eq!(o.status, OrderStatus::Delivered);
        assert!(!o.cancel());
    }
}

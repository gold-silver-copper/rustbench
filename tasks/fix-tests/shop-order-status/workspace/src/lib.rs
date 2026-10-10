//! Order lifecycle for a web shop.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pending,
    Paid,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Pay,
    Ship,
    Deliver,
    Cancel,
}

/// Pending -> Paid -> Shipped -> Delivered. Cancelling a pending order
/// cancels it; cancelling a paid order refunds it; shipped orders can't be
/// cancelled. Events that don't apply leave the status unchanged.
pub fn apply(status: Status, event: Event) -> Status {
    match (status, event) {
        (Status::Pending, Event::Pay) => Status::Paid,
        (Status::Paid, Event::Ship) => Status::Shipped,
        (Status::Shipped, Event::Deliver) => Status::Delivered,
        (Status::Pending, Event::Cancel) | (Status::Paid, Event::Cancel) => Status::Cancelled,
        (s, _) => s,
    }
}

/// The status of a new order after `events`.
pub fn replay(events: &[Event]) -> Status {
    events.iter().fold(Status::Pending, |s, &e| apply(s, e))
}

use task::{Event::*, Status, apply, replay};

#[test]
fn lifecycle() {
    assert_eq!(replay(&[]), Status::Pending);
    assert_eq!(replay(&[Cancel]), Status::Cancelled);
    assert_eq!(replay(&[Pay, Ship, Deliver]), Status::Delivered);
    assert_eq!(replay(&[Pay, Ship, Cancel]), Status::Shipped);
    assert_eq!(replay(&[Pay, Cancel, Ship]), Status::Refunded);
    assert_eq!(replay(&[Cancel, Pay]), Status::Cancelled);
}

#[test]
fn single_steps() {
    assert_eq!(apply(Status::Paid, Cancel), Status::Refunded);
    assert_eq!(apply(Status::Pending, Cancel), Status::Cancelled);
    assert_eq!(apply(Status::Refunded, Cancel), Status::Refunded);
}

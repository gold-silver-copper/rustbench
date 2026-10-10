use task::{Event, Status, replay};

#[test]
fn cancel_after_paying_refunds() {
    assert_eq!(replay(&[Event::Pay, Event::Cancel]), Status::Refunded);
}

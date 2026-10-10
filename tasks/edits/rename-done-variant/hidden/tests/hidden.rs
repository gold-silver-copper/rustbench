use task::{Order, OrderStatus};

#[test]
fn delivered_variant() {
    let mut o = Order::new(5);
    assert!(o.advance() && o.advance() && o.advance());
    assert_eq!(o.status, OrderStatus::Delivered);
    assert!(!o.advance());
    assert!(!o.cancel());
    assert_eq!(OrderStatus::Delivered.label(), "delivered");
    assert_eq!(OrderStatus::Shipped.next(), Some(OrderStatus::Delivered));
}

#[test]
fn tracking_unchanged() {
    let orders = vec![
        Order { id: 1, status: OrderStatus::Delivered },
        Order { id: 2, status: OrderStatus::Packed },
        Order { id: 3, status: OrderStatus::Cancelled },
    ];
    assert_eq!(task::tracking::delivered_count(&orders), 1);
    assert_eq!(task::tracking::open_orders(&orders), vec![2]);
}

#[test]
fn old_variant_is_gone() {
    for src in [
        include_str!("../src/status.rs"),
        include_str!("../src/order.rs"),
        include_str!("../src/tracking.rs"),
        include_str!("../src/main.rs"),
    ] {
        assert!(!src.contains("::Done"));
    }
}

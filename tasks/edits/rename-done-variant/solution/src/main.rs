use task::{Order, OrderStatus};

fn main() {
    let mut orders = vec![Order::new(10), Order::new(11), Order::new(12)];
    for _ in 0..3 {
        orders[0].advance();
    }
    orders[1].advance();
    orders[2].cancel();
    orders.push(Order { id: 13, status: OrderStatus::Delivered });
    for o in &orders {
        println!("order {}: {}", o.id, o.status.label());
    }
    println!("delivered: {}", task::tracking::delivered_count(&orders));
    println!("open: {:?}", task::tracking::open_orders(&orders));
}

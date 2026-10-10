use task::{Order, boxes_needed, shipping_cost_cents};

#[test]
fn box_counts() {
    assert_eq!(
        boxes_needed(&Order {
            items: 24,
            per_box: 12
        }),
        2
    );
    assert_eq!(
        boxes_needed(&Order {
            items: 1,
            per_box: 12
        }),
        1
    );
    assert_eq!(
        boxes_needed(&Order {
            items: 0,
            per_box: 12
        }),
        0
    );
    assert_eq!(
        boxes_needed(&Order {
            items: 7,
            per_box: 1
        }),
        7
    );
}

#[test]
fn cost() {
    assert_eq!(
        shipping_cost_cents(
            &Order {
                items: 13,
                per_box: 6
            },
            450
        ),
        1350
    );
}

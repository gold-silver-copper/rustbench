use task::{Order, boxes_needed};

#[test]
fn leftover_items_need_a_box() {
    assert_eq!(
        boxes_needed(&Order {
            items: 25,
            per_box: 12
        }),
        3
    );
}

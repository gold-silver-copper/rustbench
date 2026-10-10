use task::{Item, to_reorder};

#[test]
fn suggests_low_stock_items_still_sold() {
    let items = [
        Item {
            sku: "MUG-01",
            on_hand: 2,
            reorder_at: 5,
            discontinued: false,
        },
        Item {
            sku: "TEE-02",
            on_hand: 40,
            reorder_at: 10,
            discontinued: false,
        },
        Item {
            sku: "CAP-03",
            on_hand: 0,
            reorder_at: 3,
            discontinued: true,
        },
    ];
    assert_eq!(to_reorder(&items), vec!["MUG-01"]);
}

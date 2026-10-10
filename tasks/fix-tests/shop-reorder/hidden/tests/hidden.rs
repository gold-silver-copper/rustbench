use task::{Item, to_reorder};

fn item(sku: &'static str, on_hand: u32, reorder_at: u32, discontinued: bool) -> Item {
    Item {
        sku,
        on_hand,
        reorder_at,
        discontinued,
    }
}

#[test]
fn boundaries_and_order() {
    let items = [
        item("A", 5, 5, false),
        item("B", 6, 5, false),
        item("C", 1, 5, true),
        item("D", 0, 0, false),
    ];
    assert_eq!(to_reorder(&items), vec!["A", "D"]);
    assert_eq!(to_reorder(&[]), Vec::<&str>::new());
}

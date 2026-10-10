use task::{Inventory, Item};

#[test]
fn value_in_cents() {
    let mut inv = Inventory::default();
    assert_eq!(inv.total_value(), 0);
    inv.receive(Item::new("mug", 899, 12));
    inv.receive(Item::new("tea", 450, 30));
    assert_eq!(inv.total_value(), 899 * 12 + 450 * 30);
    inv.sell("mug", 2).unwrap();
    inv.receive(Item::new("free-sticker", 0, 100));
    assert_eq!(inv.total_value(), 899 * 10 + 450 * 30);
}

#[test]
fn large_values_do_not_overflow() {
    let inv = Inventory { items: vec![Item::new("gold", 5_000_000_000, 4_000_000)] };
    assert_eq!(inv.total_value(), 20_000_000_000_000_000);
}

#[test]
fn existing_methods_unchanged() {
    let mut inv = Inventory::default();
    inv.receive(Item::new("a", 1, 2));
    inv.receive(Item::new("a", 1, 3));
    assert_eq!(inv.units(), 5);
    assert_eq!(inv.sell("a", 6), Err("only 5 of a left".to_string()));
    assert_eq!(inv.sell("b", 1), Err("no such item b".to_string()));
}

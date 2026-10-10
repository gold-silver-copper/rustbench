use task::{StockItem, Warehouse};

#[test]
fn stock_item_works() {
    let a: StockItem = StockItem::new("Z9", "nut", 7, 3);
    assert_eq!(a.stock_value_cents(), 21);
    let mut w = Warehouse::new();
    w.add(a.clone());
    w.add(StockItem::new("Z9", "nut", 1, 3));
    w.add(StockItem::new("Q1", "gear", 2, 100));
    assert_eq!(w.get("Z9").map(|i| i.quantity), Some(8));
    assert_eq!(w.total_value_cents(), 224);
    let low: Vec<&StockItem> = task::restock::low_stock(&w, 5);
    assert_eq!(low.len(), 1);
    assert_eq!(task::restock::order_amount(low[0], 5), 3);
}

#[test]
fn old_name_is_gone() {
    for src in [
        include_str!("../src/lib.rs"),
        include_str!("../src/item.rs"),
        include_str!("../src/warehouse.rs"),
        include_str!("../src/restock.rs"),
        include_str!("../src/main.rs"),
    ] {
        let ident = |c: char| c.is_alphanumeric() || c == '_';
        for (i, _) in src.match_indices("Item") {
            let before = src[..i].chars().last().is_some_and(ident);
            let after = src[i + 4..].chars().next().is_some_and(ident);
            assert!(before || after, "a bare `Item` is left");
        }
    }
}

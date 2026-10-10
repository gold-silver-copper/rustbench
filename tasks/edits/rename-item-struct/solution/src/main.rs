use task::{StockItem, Warehouse};

fn main() {
    let mut w = Warehouse::new();
    let stock: Vec<StockItem> = vec![
        StockItem::new("B7", "hinge", 12, 250),
        StockItem::new("C3", "washer", 3, 5),
        StockItem::new("B7", "hinge", 2, 250),
    ];
    for item in stock {
        w.add(item);
    }
    println!("total value: {} cents", w.total_value_cents());
    for item in task::restock::low_stock(&w, 5) {
        println!("reorder {} ({}): {}", item.name, item.sku, task::restock::order_amount(item, 20));
    }
}

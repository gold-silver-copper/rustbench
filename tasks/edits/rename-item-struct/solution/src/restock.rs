use crate::item::StockItem;
use crate::warehouse::Warehouse;

/// Items whose quantity is below `threshold`, lowest first.
pub fn low_stock(w: &Warehouse, threshold: u32) -> Vec<&StockItem> {
    let mut low: Vec<&StockItem> = w.items().iter().filter(|i| i.quantity < threshold).collect();
    low.sort_by_key(|i| i.quantity);
    low
}

/// How many units to order to bring `item` up to `target`.
pub fn order_amount(item: &StockItem, target: u32) -> u32 {
    target.saturating_sub(item.quantity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_up_to_target() {
        let item = StockItem::new("A1", "bolt", 4, 10);
        assert_eq!(order_amount(&item, 10), 6);
        assert_eq!(order_amount(&item, 2), 0);
    }
}

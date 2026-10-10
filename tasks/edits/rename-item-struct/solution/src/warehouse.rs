use crate::item::StockItem;

#[derive(Debug, Default)]
pub struct Warehouse {
    items: Vec<StockItem>,
}

impl Warehouse {
    pub fn new() -> Warehouse {
        Warehouse::default()
    }

    pub fn add(&mut self, item: StockItem) {
        match self.items.iter_mut().find(|i| i.sku == item.sku) {
            Some(existing) => existing.quantity += item.quantity,
            None => self.items.push(item),
        }
    }

    pub fn get(&self, sku: &str) -> Option<&StockItem> {
        self.items.iter().find(|i| i.sku == sku)
    }

    pub fn items(&self) -> &[StockItem] {
        &self.items
    }

    pub fn total_value_cents(&self) -> u64 {
        self.items.iter().map(StockItem::stock_value_cents).sum()
    }
}

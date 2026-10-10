//! Packing orders into shipping boxes.

pub struct Order {
    pub items: u32,
    pub per_box: u32,
}

/// Boxes needed to ship every item (at most one box is partly filled).
pub fn boxes_needed(order: &Order) -> u32 {
    if order.per_box == 0 {
        return 0;
    }
    order.items / order.per_box
}

pub fn shipping_cost_cents(order: &Order, cents_per_box: u32) -> u32 {
    boxes_needed(order) * cents_per_box
}

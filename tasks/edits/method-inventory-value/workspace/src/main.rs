use task::{Inventory, Item};

fn main() {
    let mut inv = Inventory::default();
    inv.receive(Item::new("mug", 899, 12));
    inv.receive(Item::new("tea", 450, 30));
    inv.receive(Item::new("mug", 899, 3));
    if let Err(e) = inv.sell("tea", 40) {
        println!("{e}");
    }
    println!("{} units", inv.units());
}

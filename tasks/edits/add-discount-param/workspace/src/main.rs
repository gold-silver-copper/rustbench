use task::cart::{self, LineItem};
use task::checkout;

fn main() {
    let items = vec![
        LineItem::new("notebook", 450, 3),
        LineItem::new("stapler", 1299, 1),
    ];
    println!("subtotal: {}", cart::total(&items));
    print!("{}", checkout::summary(&items));
}

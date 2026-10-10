use task::invoice::{Invoice, Line};
use task::pricing;

fn main() {
    let inv = Invoice {
        lines: vec![
            Line { item: "pen".into(), qty: 3, unit_cents: 150 },
            Line { item: "pad".into(), qty: 1, unit_cents: 499 },
        ],
    };
    println!("{}", inv.footer());
    println!("a 10.00 item costs {} cents", pricing::gross(1000));
}

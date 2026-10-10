use task::Line;
use task::invoice::render_invoice;
use task::receipt::render_receipt;

fn main() {
    let lines = vec![
        Line::new("consulting", 125_000),
        Line::new("travel", 8_940),
        Line::new("discount", -5_000),
    ];
    print!("{}", render_invoice("Acme Ltd", &lines));
    println!("{}", render_receipt(&lines));
}

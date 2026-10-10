use task::Shape;
use task::describe::{describe, total_area};

fn main() {
    let shapes = [
        Shape::Circle { r: 1.0 },
        Shape::Rect { w: 2.0, h: 3.0 },
        Shape::Rect { w: 1.5, h: 1.5 }.scaled(2.0),
    ];
    for s in &shapes {
        println!("{}", describe(s));
    }
    println!("total area {:.2}", total_area(&shapes));
}

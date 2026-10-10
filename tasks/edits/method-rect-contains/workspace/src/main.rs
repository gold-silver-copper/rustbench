use task::Rect;
use task::layout::columns;

fn main() {
    for c in columns(Rect::new(0, 0, 100, 20), 3) {
        println!("{:?} area {}", c.origin, c.area());
    }
}

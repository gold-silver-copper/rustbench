use task::board::Board;

fn main() {
    let mut b = Board::default();
    b.record("ada", 120);
    b.record("grace", 340);
    b.record("linus", 95);
    print!("{}", b.top(2));
}

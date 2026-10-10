use task::{Block, busy};

fn b(start: u32, end: u32) -> Block {
    Block { start, end }
}

#[test]
fn touching_and_separate() {
    assert_eq!(
        busy(&[b(60, 90), b(0, 30), b(30, 45)]),
        vec![b(0, 45), b(60, 90)]
    );
}

#[test]
fn nested_out_of_order() {
    assert_eq!(
        busy(&[b(100, 200), b(0, 300), b(400, 500)]),
        vec![b(0, 300), b(400, 500)]
    );
    assert_eq!(busy(&[b(10, 20), b(5, 50), b(40, 60)]), vec![b(5, 60)]);
    assert_eq!(busy(&[]), vec![]);
}

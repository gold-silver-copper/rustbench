use task::Tile;

#[test]
fn distinct() {
    let a = Tile { x: 0, y: 0 };
    let b = Tile { x: 1, y: 0 };
    assert_eq!(task::distinct_tiles(&[a, b, a, a]), 2);
    assert_eq!(task::distinct_tiles(&[]), 0);
}

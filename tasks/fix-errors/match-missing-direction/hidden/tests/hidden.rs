use task::{step, turn_right, Heading};

#[test]
fn walks() {
    let mut pos = (0, 0);
    let mut h = Heading::North;
    for _ in 0..4 {
        pos = step(pos, h);
        h = turn_right(h);
    }
    assert_eq!(pos, (0, 0));
    assert_eq!(step((5, 5), Heading::West), (4, 5));
    assert_eq!(step((5, 5), Heading::South), (5, 4));
}

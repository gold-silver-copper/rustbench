use task::{Player, standings};

fn p(name: &str, points: u32, wins: u32) -> Player {
    Player {
        name: name.into(),
        points,
        wins,
    }
}

#[test]
fn full_table() {
    let players = [
        p("cara", 4, 1),
        p("bea", 4, 3),
        p("abe", 4, 1),
        p("dan", 9, 0),
        p("eve", 0, 0),
    ];
    assert_eq!(
        standings(&players),
        vec!["dan", "bea", "abe", "cara", "eve"]
    );
    assert_eq!(standings(&[]), Vec::<String>::new());
}

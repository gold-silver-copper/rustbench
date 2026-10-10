use task::{Player, standings};

fn p(name: &str, points: u32, wins: u32) -> Player {
    Player {
        name: name.into(),
        points,
        wins,
    }
}

#[test]
fn ties_go_alphabetical() {
    let players = [p("zoe", 5, 2), p("adam", 5, 2), p("li", 6, 1)];
    assert_eq!(standings(&players), vec!["li", "adam", "zoe"]);
}

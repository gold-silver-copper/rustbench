use task::{Score, top};

fn s(player: &str, points: u32) -> Score {
    Score {
        player: player.into(),
        points,
    }
}

#[test]
fn top_two_scores() {
    let scores = [s("ann", 300), s("bo", 900), s("cy", 500)];
    assert_eq!(top(&scores, 2), vec![s("bo", 900), s("cy", 500)]);
}

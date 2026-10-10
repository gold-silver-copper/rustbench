use task::{Score, top};

fn s(player: &str, points: u32) -> Score {
    Score {
        player: player.into(),
        points,
    }
}

#[test]
fn ties_by_name() {
    let scores = [s("dee", 500), s("cy", 500), s("bo", 900), s("al", 10)];
    assert_eq!(
        top(&scores, 3),
        vec![s("bo", 900), s("cy", 500), s("dee", 500)]
    );
}

#[test]
fn fewer_than_n() {
    assert_eq!(top(&[s("x", 1)], 5), vec![s("x", 1)]);
    assert_eq!(top(&[], 5), Vec::<Score>::new());
}

use task::{Action, History};

#[test]
fn more_than_recorded() {
    let mut h = History::default();
    h.record(Action::Draw("a".into()));
    h.record(Action::Move {
        shape: "a".into(),
        dx: 3,
        dy: -1,
    });
    assert_eq!(
        h.recent(10),
        vec![
            &Action::Move {
                shape: "a".into(),
                dx: 3,
                dy: -1
            },
            &Action::Draw("a".into())
        ]
    );
    assert!(h.recent(0).is_empty());
    assert_eq!(h.len(), 2);
}

#[test]
fn newest_one() {
    let mut h = History::default();
    for i in 0..5 {
        h.record(Action::Draw(format!("s{i}")));
    }
    assert_eq!(h.recent(1), vec![&Action::Draw("s4".into())]);
}

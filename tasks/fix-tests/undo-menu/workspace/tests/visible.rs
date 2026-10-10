use task::{Action, History};

#[test]
fn recent_lists_newest_first() {
    let mut h = History::default();
    h.record(Action::Draw("circle".into()));
    h.record(Action::Draw("square".into()));
    h.record(Action::Erase("circle".into()));
    assert_eq!(
        h.recent(2),
        vec![
            &Action::Erase("circle".into()),
            &Action::Draw("square".into())
        ]
    );
}

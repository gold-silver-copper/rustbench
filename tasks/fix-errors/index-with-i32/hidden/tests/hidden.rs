#[test]
fn ranks() {
    let p = ["ann", "bob", "cy"];
    assert_eq!(task::player_at_rank(&p, 1).as_deref(), Some("ann"));
    assert_eq!(task::player_at_rank(&p, 3).as_deref(), Some("cy"));
    assert_eq!(task::player_at_rank(&p, 0), None);
    assert_eq!(task::player_at_rank(&p, 4), None);
    assert_eq!(task::player_at_rank(&p, -2), None);
}

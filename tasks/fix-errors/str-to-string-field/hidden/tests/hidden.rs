#[test]
fn lifecycle() {
    let mut t = task::Ticket::new("login page", 3);
    assert_eq!(t.status, "open");
    assert_eq!(t.title, "login page");
    t.close();
    assert_eq!(t.status, "closed");
    assert_eq!(t.points, 3);
}

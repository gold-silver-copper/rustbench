use task::Contact;

#[test]
fn addresses() {
    let a = Contact { name: "Ana".into(), email: Some("ana@example.com".into()) };
    assert_eq!(a.address(), "ana@example.com");
    assert_eq!(a.address(), "ana@example.com");
    assert_eq!(a.email.as_deref(), Some("ana@example.com"));
    let b = Contact { name: "Bo".into(), email: None };
    assert_eq!(b.address(), "no email on file for Bo");
}

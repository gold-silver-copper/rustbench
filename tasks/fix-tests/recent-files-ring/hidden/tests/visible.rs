use task::Recent;

#[test]
fn keeps_the_last_three() {
    let mut r = Recent::new(3);
    for f in ["a.rs", "b.rs", "c.rs", "d.rs"] {
        r.open(f);
    }
    assert_eq!(r.files(), vec!["b.rs", "c.rs", "d.rs"]);
}

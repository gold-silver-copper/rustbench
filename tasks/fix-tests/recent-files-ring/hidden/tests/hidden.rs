use task::Recent;

#[test]
fn two_slots() {
    let mut r = Recent::new(2);
    r.open("x");
    r.open("y");
    assert_eq!(r.files(), vec!["x", "y"]);
    r.open("z");
    assert_eq!(r.files(), vec!["y", "z"]);
}

#[test]
fn not_yet_full_and_single_slot() {
    let mut r = Recent::new(4);
    r.open("a");
    r.open("b");
    assert_eq!(r.files(), vec!["a", "b"]);
    let mut one = Recent::new(1);
    one.open("p");
    one.open("q");
    assert_eq!(one.files(), vec!["q"]);
}

#[test]
fn wraps_many_times() {
    let mut r = Recent::new(5);
    for i in 0..23 {
        r.open(&format!("f{i}"));
    }
    assert_eq!(r.files(), vec!["f18", "f19", "f20", "f21", "f22"]);
}

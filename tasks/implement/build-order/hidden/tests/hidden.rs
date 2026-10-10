use task::build_order;

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

#[test]
fn orders() {
    assert_eq!(build_order(&[], &[]), Ok(vec![]));
    assert_eq!(build_order(&["b", "a", "c", "a"], &[]), Ok(s(&["a", "b", "c"])));
    assert_eq!(build_order(&[], &[("a", "b")]), Ok(s(&["b", "a"])));
    // After `d`, both `b` and `c` are ready; `a` waits for both.
    assert_eq!(
        build_order(&[], &[("a", "b"), ("a", "c"), ("b", "d"), ("c", "d"), ("a", "b")]),
        Ok(s(&["d", "b", "c", "a"]))
    );
    // `z` is ready from the start but `m` becomes ready before it is placed.
    assert_eq!(build_order(&["z"], &[("m", "a")]), Ok(s(&["a", "m", "z"])));
}

#[test]
fn cycles() {
    assert_eq!(build_order(&[], &[("a", "a")]), Err(s(&["a"])));
    assert_eq!(
        build_order(&["solo"], &[("x", "y"), ("y", "x"), ("after", "x"), ("y", "base")]),
        Err(s(&["after", "x", "y"]))
    );
}

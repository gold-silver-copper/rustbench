use task::{eval::eval, parse::tokenize};

#[test]
fn nested_expression() {
    let t = tokenize("2 3 4 * +").unwrap_or_default();
    assert_eq!(eval(&t), Some(14));
}

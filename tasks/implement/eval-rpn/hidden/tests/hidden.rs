use task::{eval_rpn, RpnError::*};

#[test]
fn evaluates() {
    assert_eq!(eval_rpn("42"), Ok(42));
    assert_eq!(eval_rpn("  5\t1 2 + 4 * + 3 -  "), Ok(14));
    assert_eq!(eval_rpn("10 3 -"), Ok(7));
    assert_eq!(eval_rpn("-7 2 /"), Ok(-3));
    assert_eq!(eval_rpn("-7 2 %"), Ok(-1));
    assert_eq!(eval_rpn("7 -2 %"), Ok(1));
    assert_eq!(eval_rpn("+3 -3 *"), Ok(-9));
}

#[test]
fn errors() {
    assert_eq!(eval_rpn("1 2 x +"), Err(BadToken(2)));
    assert_eq!(eval_rpn("1 2 ++"), Err(BadToken(2)));
    assert_eq!(eval_rpn("99999999999999999999"), Err(BadToken(0)));
    assert_eq!(eval_rpn("+"), Err(Underflow(0)));
    assert_eq!(eval_rpn("1 2 + *"), Err(Underflow(3)));
    assert_eq!(eval_rpn("5 0 %"), Err(DivByZero(2)));
    assert_eq!(eval_rpn("1 0 / x"), Err(DivByZero(2)));
    assert_eq!(eval_rpn("9223372036854775807 1 +"), Err(Overflow(2)));
    assert_eq!(eval_rpn("-9223372036854775808 -1 /"), Err(Overflow(2)));
    assert_eq!(eval_rpn("-9223372036854775808 -1 %"), Err(Overflow(2)));
}

#[test]
fn leftover() {
    assert_eq!(eval_rpn(""), Err(Leftover(0)));
    assert_eq!(eval_rpn("   "), Err(Leftover(0)));
    assert_eq!(eval_rpn("1 2 3 +"), Err(Leftover(2)));
}

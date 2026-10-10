use task::{tokenize, Token, Token::*};

fn op(s: &str) -> Token {
    Op(s.to_string())
}
fn id(s: &str) -> Token {
    Ident(s.to_string())
}

#[test]
fn tokens() {
    assert_eq!(
        tokenize("_a1+b_2*(c-007)/9"),
        Ok(vec![id("_a1"), op("+"), id("b_2"), op("*"), LParen, id("c"), op("-"), Num(7), RParen, op("/"), Num(9)])
    );
    assert_eq!(tokenize("\tx\n=\r\n1 "), Ok(vec![id("x"), op("="), Num(1)]));
    assert_eq!(tokenize("1,2"), Ok(vec![Num(1), Comma, Num(2)]));
    assert_eq!(tokenize("18446744073709551615"), Ok(vec![Num(u64::MAX)]));
}

#[test]
fn two_char_operators() {
    assert_eq!(tokenize("a<=b"), Ok(vec![id("a"), op("<="), id("b")]));
    assert_eq!(tokenize("==="), Ok(vec![op("=="), op("=")]));
    assert_eq!(tokenize("!==<>="), Ok(vec![op("!="), op("="), op("<"), op(">=")]));
    assert_eq!(tokenize("< ="), Ok(vec![op("<"), op("=")]));
}

#[test]
fn empty() {
    assert_eq!(tokenize(""), Ok(vec![]));
    assert_eq!(tokenize("  \n\t"), Ok(vec![]));
}

#[test]
fn errors() {
    assert_eq!(tokenize("x = 12ab"), Err(6));
    assert_eq!(tokenize("3_"), Err(1));
    assert_eq!(tokenize("1 + 18446744073709551616"), Err(4));
    assert_eq!(tokenize("a != !b"), Err(5));
    assert_eq!(tokenize("!"), Err(0));
    assert_eq!(tokenize("x;"), Err(1));
    assert_eq!(tokenize("é"), Err(0));
    assert_eq!(tokenize("aé"), Err(1));
    assert_eq!(tokenize("é é"), Err(0));
}

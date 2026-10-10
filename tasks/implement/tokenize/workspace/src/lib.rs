/// One token of the expression language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A non-negative decimal integer literal.
    Num(u64),
    /// An identifier.
    Ident(String),
    /// An operator: one of `+ - * / = < > == != <= >=`.
    Op(String),
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `,`
    Comma,
}

/// Splits `src` into tokens.
///
/// - ASCII whitespace between tokens is skipped.
/// - A number is a maximal run of ASCII digits, read as a `u64`. Leading
///   zeros are allowed (`007` is `Num(7)`).
/// - An identifier starts with an ASCII letter or `_` and continues with
///   ASCII letters, ASCII digits and `_`.
/// - `==`, `!=`, `<=` and `>=` are always read as one two-character operator
///   (`a<=b` is `a`, `<=`, `b`; `===` is `==` then `=`). Otherwise `+ - * /
///   = < >` are one-character operators.
/// - `(`, `)` and `,` are tokens of their own.
///
/// On error, returns `Err(offset)`, a byte offset into `src`:
/// - a number immediately followed by an ASCII letter or `_` (as in `12ab`):
///   the offset of that letter or `_`;
/// - a number too large for `u64`: the offset of its first digit;
/// - a `!` not followed by `=`, or any character not covered above
///   (including non-ASCII characters): the offset of that character.
///
/// Input that is empty or only whitespace gives `Ok(vec![])`.
///
/// ```
/// use task::Token::*;
/// assert_eq!(
///     task::tokenize("f(x, 10) >= 3"),
///     Ok(vec![Ident("f".into()), LParen, Ident("x".into()), Comma, Num(10), RParen, Op(">=".into()), Num(3)])
/// );
/// assert_eq!(task::tokenize("a # b"), Err(2));
/// ```
pub fn tokenize(src: &str) -> Result<Vec<Token>, usize> {
    todo!()
}

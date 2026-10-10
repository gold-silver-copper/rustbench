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
    let bytes = src.as_bytes();
    let text = |a: usize, b: usize| src.get(a..b).unwrap_or_default().to_string();
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(&b) = bytes.get(i) {
        let start = i;
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b.is_ascii_digit() {
            while bytes.get(i).is_some_and(u8::is_ascii_digit) {
                i += 1;
            }
            if bytes.get(i).is_some_and(|c| c.is_ascii_alphabetic() || *c == b'_') {
                return Err(i);
            }
            let n = text(start, i).parse::<u64>().map_err(|_| start)?;
            out.push(Token::Num(n));
            continue;
        }
        if b.is_ascii_alphabetic() || b == b'_' {
            while bytes.get(i).is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_') {
                i += 1;
            }
            out.push(Token::Ident(text(start, i)));
            continue;
        }
        let tok = match (b, bytes.get(i + 1)) {
            (b'=' | b'!' | b'<' | b'>', Some(b'=')) => {
                i += 2;
                Token::Op(text(start, i))
            }
            (b'+' | b'-' | b'*' | b'/' | b'=' | b'<' | b'>', _) => {
                i += 1;
                Token::Op(text(start, i))
            }
            (b'(', _) => {
                i += 1;
                Token::LParen
            }
            (b')', _) => {
                i += 1;
                Token::RParen
            }
            (b',', _) => {
                i += 1;
                Token::Comma
            }
            _ => return Err(start),
        };
        out.push(tok);
    }
    Ok(out)
}

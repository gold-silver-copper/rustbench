/// Why [`eval_rpn`] failed. Token indexes count whitespace-separated tokens
/// from 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpnError {
    /// The token at this index is neither an integer nor an operator.
    BadToken(usize),
    /// The operator at this index found fewer than two values on the stack.
    Underflow(usize),
    /// The `/` or `%` at this index has a right operand of 0.
    DivByZero(usize),
    /// The operator at this index overflowed `i64`.
    Overflow(usize),
    /// After the last token the stack does not hold exactly one value;
    /// carries how many values it holds.
    Leftover(usize),
}

/// Evaluates a reverse Polish expression over `i64`.
///
/// Tokens are separated by whitespace and processed left to right with a
/// stack. A token that `str::parse::<i64>` accepts is a number and is pushed
/// (so `-3` and `+3` are numbers). Otherwise it must be one of the operators
/// `+ - * / %`: it pops the right operand, then the left operand, and pushes
/// `left op right`. `/` truncates toward zero and `%` takes the sign of the
/// left operand, as Rust's `/` and `%` do.
///
/// The first failing token decides the error. At the end, the single value
/// left on the stack is the result; an empty stack or more than one value
/// gives `Leftover(n)` (so empty input is `Leftover(0)`).
///
/// ```
/// assert_eq!(task::eval_rpn("3 4 + 2 *"), Ok(14));
/// assert_eq!(task::eval_rpn("1 0 /"), Err(task::RpnError::DivByZero(2)));
/// ```
pub fn eval_rpn(expr: &str) -> Result<i64, RpnError> {
    let mut stack: Vec<i64> = Vec::new();
    for (i, tok) in expr.split_whitespace().enumerate() {
        if let Ok(n) = tok.parse::<i64>() {
            stack.push(n);
            continue;
        }
        if !matches!(tok, "+" | "-" | "*" | "/" | "%") {
            return Err(RpnError::BadToken(i));
        }
        let (Some(r), Some(l)) = (stack.pop(), stack.pop()) else {
            return Err(RpnError::Underflow(i));
        };
        let v = match tok {
            "+" => l.checked_add(r),
            "-" => l.checked_sub(r),
            "*" => l.checked_mul(r),
            _ if r == 0 => return Err(RpnError::DivByZero(i)),
            "/" => l.checked_div(r),
            _ => l.checked_rem(r),
        };
        stack.push(v.ok_or(RpnError::Overflow(i))?);
    }
    match stack.as_slice() {
        [v] => Ok(*v),
        s => Err(RpnError::Leftover(s.len())),
    }
}

use crate::parse::Token;

/// Evaluates reverse Polish notation.
pub fn eval(tokens: &[Token]) -> Option<i64> {
    let mut stack = Vec::new();
    for t in tokens {
        match t {
            Token::Num(n) => stack.push(*n),
            Token::Op(op) => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(match op {
                    '+' => a + b,
                    '-' => a - b,
                    _ => a * b,
                });
            }
        }
    }
    if stack.len() == 1 { stack.pop() } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::tokenize;

    #[test]
    fn adds() {
        assert_eq!(tokenize("2 3 +").and_then(|t| eval(&t)), Some(5));
    }

    #[test]
    fn underflow_is_none() {
        assert_eq!(tokenize("+").and_then(|t| eval(&t)), None);
    }

    #[test]
    #[ignore = "overflow behavior not decided yet"]
    fn huge_numbers() {
        assert_eq!(tokenize("9223372036854775807 1 +").and_then(|t| eval(&t)), None);
    }
}

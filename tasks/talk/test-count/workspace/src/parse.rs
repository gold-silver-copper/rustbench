#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(i64),
    Op(char),
}

pub fn tokenize(src: &str) -> Option<Vec<Token>> {
    src.split_whitespace()
        .map(|w| match w {
            "+" | "-" | "*" => w.chars().next().map(Token::Op),
            _ => w.parse().ok().map(Token::Num),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(s: &str) -> Vec<Token> {
        tokenize(s).unwrap_or_default()
    }

    #[test]
    fn numbers_and_ops() {
        assert_eq!(toks("1 2 +"), vec![Token::Num(1), Token::Num(2), Token::Op('+')]);
    }

    #[test]
    fn rejects_words() {
        assert_eq!(tokenize("1 two +"), None);
    }

    // #[test]
    // fn handles_parens() {}
}

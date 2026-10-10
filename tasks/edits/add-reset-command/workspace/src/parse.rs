use crate::command::Command;

/// Parse one input line: "inc [n]", "dec [n]" or "show". `n` defaults to 1.
pub fn parse(line: &str) -> Option<Command> {
    let mut words = line.split_whitespace();
    let word = words.next()?;
    let arg = words.next();
    if words.next().is_some() {
        return None;
    }
    let amount = || match arg {
        None => Some(1),
        Some(a) => a.parse::<i64>().ok(),
    };
    match word {
        "inc" => Some(Command::Inc(amount()?)),
        "dec" => Some(Command::Dec(amount()?)),
        "show" if arg.is_none() => Some(Command::Show),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_words() {
        assert_eq!(parse("inc"), Some(Command::Inc(1)));
        assert_eq!(parse("  dec 5 "), Some(Command::Dec(5)));
        assert_eq!(parse("show"), Some(Command::Show));
        assert_eq!(parse("show 2"), None);
        assert_eq!(parse("jump"), None);
    }
}

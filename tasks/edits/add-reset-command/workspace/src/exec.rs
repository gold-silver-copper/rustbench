use crate::command::Command;

/// Apply a command to the counter; returns the line to echo back.
pub fn apply(counter: &mut i64, cmd: Command) -> String {
    match cmd {
        Command::Inc(n) => {
            *counter += n;
            format!("+{n}")
        }
        Command::Dec(n) => {
            *counter -= n;
            format!("-{n}")
        }
        Command::Show => format!("{counter}"),
    }
}

/// Run every line of a script, returning the echoed lines.
/// Lines that don't parse echo "?".
pub fn run_script(script: &str) -> (i64, Vec<String>) {
    let mut counter = 0;
    let mut out = Vec::new();
    for line in script.lines().filter(|l| !l.trim().is_empty()) {
        match crate::parse::parse(line) {
            Some(cmd) => out.push(apply(&mut counter, cmd)),
            None => out.push("?".to_string()),
        }
    }
    (counter, out)
}

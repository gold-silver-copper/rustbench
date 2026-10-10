/// The five commands the todo CLI understands.
#[derive(Debug, PartialEq)]
pub enum Command {
    Add { title: String, due: Option<String> },
    Remove(u32),
    List,
    Done(u32),
    // Rename(u32, String),
    Undo,
    Clear,
}

pub fn parse(line: &str) -> Option<Command> {
    let mut words = line.split_whitespace();
    let cmd = match words.next()? {
        "add" => Command::Add {
            title: words.collect::<Vec<_>>().join(" "),
            due: None,
        },
        "rm" => Command::Remove(words.next()?.parse().ok()?),
        "ls" => Command::List,
        "done" => Command::Done(words.next()?.parse().ok()?),
        "undo" => Command::Undo,
        "clear" => Command::Clear,
        _ => return None,
    };
    Some(cmd)
}

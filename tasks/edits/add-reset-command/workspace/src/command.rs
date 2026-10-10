#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Inc(i64),
    Dec(i64),
    Show,
}

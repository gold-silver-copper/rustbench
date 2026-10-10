use task::Command;
use task::exec::{apply, run_script};
use task::parse::parse;

#[test]
fn parses_reset() {
    assert_eq!(parse("reset"), Some(Command::Reset));
    assert_eq!(parse("  reset  "), Some(Command::Reset));
    assert_eq!(parse("reset 3"), None);
    assert_eq!(parse("inc 2"), Some(Command::Inc(2)));
    assert_eq!(parse("show"), Some(Command::Show));
}

#[test]
fn reset_zeroes_counter() {
    let mut c = 42;
    apply(&mut c, Command::Reset);
    assert_eq!(c, 0);
    let (end, out) = run_script("inc 5\ndec 2\nreset\ninc\nshow\nbogus\n");
    assert_eq!(end, 1);
    assert_eq!(out.len(), 6);
    assert_eq!(out[0], "+5");
    assert_eq!(out[4], "1");
    assert_eq!(out[5], "?");
}

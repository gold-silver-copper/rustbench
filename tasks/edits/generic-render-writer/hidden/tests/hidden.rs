use std::fmt;

use task::Row;
use task::board::Board;
use task::table::render_table;

fn rows() -> Vec<Row> {
    vec![
        Row { name: "grace".into(), score: 340 },
        Row { name: "ada".into(), score: 120 },
    ]
}

const EXPECTED: &str = "name   score\n------------\ngrace    340\nada      120\n";

#[test]
fn renders_into_string() {
    let mut s = String::new();
    let r: fmt::Result = render_table(&mut s, &rows());
    assert!(r.is_ok());
    assert_eq!(s, EXPECTED);
}

struct Counter(usize);

impl fmt::Write for Counter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0 += s.len();
        Ok(())
    }
}

#[test]
fn renders_into_any_writer() {
    let mut c = Counter(0);
    render_table(&mut c, &rows()).unwrap();
    assert_eq!(c.0, EXPECTED.len());
}

struct Full;

impl fmt::Write for Full {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Err(fmt::Error)
    }
}

#[test]
fn errors_are_returned() {
    assert!(render_table(&mut Full, &rows()).is_err());
}

#[test]
fn board_unchanged() {
    let mut b = Board::default();
    b.record("ada", 120);
    b.record("grace", 340);
    b.record("linus", 95);
    assert_eq!(b.top(2), EXPECTED);
    assert_eq!(b.top(0), "name  score\n-----------\n");
}

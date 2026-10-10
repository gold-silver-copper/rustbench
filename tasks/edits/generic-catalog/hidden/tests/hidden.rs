use task::{Book, Describe, Movie, catalog};

#[test]
fn movies() {
    let films = [
        Movie { title: "Alien".into(), year: 1979, minutes: 117 },
        Movie { title: "Heat".into(), year: 1995, minutes: 170 },
    ];
    assert_eq!(catalog(&films), "1. Alien (1979, 117 min)\n2. Heat (1995, 170 min)\n");
}

#[test]
fn books_unchanged() {
    let shelf = [Book { title: "Dune".into(), author: "Frank Herbert".into(), pages: 412 }];
    assert_eq!(catalog(&shelf), "1. Dune by Frank Herbert (412 pages)\n");
}

struct Note(&'static str);
impl Describe for Note {
    fn describe(&self) -> String {
        self.0.to_string()
    }
}

#[test]
fn any_describe() {
    assert_eq!(catalog(&[Note("a"), Note("b")]), "1. a\n2. b\n");
    assert_eq!(catalog::<Note>(&[]), "");
}

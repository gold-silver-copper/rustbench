use task::book::Book;

#[test]
fn older() {
    let shelf = vec![
        Book { title: "Dune".into(), year: 1965 },
        Book { title: "Emma".into(), year: 1815 },
        Book { title: "Neuromancer".into(), year: 1984 },
    ];
    let old = task::library::older_than(&shelf, 1970);
    let titles: Vec<&str> = old.iter().map(|b| b.title.as_str()).collect();
    assert_eq!(titles, ["Emma", "Dune"]);
    assert_eq!(shelf.len(), 3);
}

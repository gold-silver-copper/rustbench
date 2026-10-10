#[test]
fn picks_longer() {
    assert_eq!(task::longer_title("Dune", "Emma"), "Dune");
    assert_eq!(task::longer_title("It", "Persuasion"), "Persuasion");
    let owned = String::from("Middlemarch");
    assert_eq!(task::longer_title(&owned, "Ulysses"), "Middlemarch");
}

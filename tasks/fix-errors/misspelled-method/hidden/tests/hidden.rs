#[test]
fn counts() {
    let names: Vec<String> = ["Al", "Beatrice", "Cy", "Dominique"].iter().map(|s| s.to_string()).collect();
    assert_eq!(task::long_names(&names, 3), 2);
    assert_eq!(task::long_names(&names, 100), 0);
}

use task::page;

#[test]
fn first_page_shows_three_items() {
    let results = ["a", "b", "c", "d", "e", "f", "g"];
    assert_eq!(page(&results, 0, 3), vec!["a", "b", "c"]);
}

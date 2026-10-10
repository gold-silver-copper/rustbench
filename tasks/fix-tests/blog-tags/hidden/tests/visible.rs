use task::normalize_tags;

#[test]
fn duplicate_tags_are_merged() {
    assert_eq!(
        normalize_tags(&["Rust", "web", " rust "]),
        vec!["rust", "web"]
    );
}

use task::Note;
use task::search::tagged;
use task::tags::{add_tag, normalize_tag, remove_tag};

#[test]
fn normalizes() {
    assert_eq!(normalize_tag("  To Do! "), "to-do");
    assert_eq!(normalize_tag("Rust 2024"), "rust-2024");
    assert_eq!(normalize_tag("café"), "caf");
    assert_eq!(normalize_tag("a_b.c"), "abc");
    assert_eq!(normalize_tag("   "), "");
}

#[test]
fn add_and_search_unchanged() {
    let mut a = Note { title: "A".into(), ..Note::default() };
    add_tag(&mut a, " Home Work ");
    add_tag(&mut a, "home work");
    add_tag(&mut a, "!!!");
    assert_eq!(a.tags, vec!["home-work".to_string()]);
    let notes = [a.clone()];
    assert_eq!(tagged(&notes, "HOME WORK"), vec!["A"]);
    assert!(tagged(&notes, "home").is_empty());
    assert!(remove_tag(&mut a, "home-work"));
    assert!(a.tags.is_empty());
}

#[test]
fn duplication_gone() {
    let search = include_str!("../src/search.rs");
    assert!(search.contains("normalize_tag"));
    assert!(!search.contains("is_ascii_alphanumeric"));
    assert_eq!(include_str!("../src/tags.rs").matches("is_ascii_alphanumeric").count(), 1);
}

use task::loader::parse_line;

#[test]
fn parses() {
    let a = parse_line("BOLT-10 42 20").unwrap();
    assert_eq!((a.sku.as_str(), a.quantity, a.reorder_level), ("BOLT-10", 42, 20));
    let b = parse_line("NUT 3").unwrap();
    assert_eq!(b.reorder_level, 5);
    assert!(b.needs_reorder());
    assert!(parse_line("NUT x").is_none());
}

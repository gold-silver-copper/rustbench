#[test]
fn confirmation() {
    let (order, line) = task::place_order("Bo".into(), vec!["a".into()]);
    assert_eq!(line, "Order for Bo with 1 item(s)");
    assert_eq!(order.customer, "Bo");
    assert_eq!(order.items, vec!["a".to_string()]);
}

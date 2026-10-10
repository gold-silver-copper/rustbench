use task::Product;

#[test]
fn labels() {
    let p = Product { name: "Tea".into(), price_cents: 350 };
    assert_eq!(p.label(), "Tea - $3.50");
    let p = Product { name: "Gum".into(), price_cents: 5 };
    assert_eq!(p.label(), "Gum - $0.05");
}

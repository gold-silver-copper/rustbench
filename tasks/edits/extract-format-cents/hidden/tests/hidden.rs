use task::Line;
use task::invoice::render_invoice;
use task::money::format_cents;
use task::receipt::render_receipt;

#[test]
fn formats_cents() {
    assert_eq!(format_cents(1205), "$12.05");
    assert_eq!(format_cents(-340), "-$3.40");
    assert_eq!(format_cents(0), "$0.00");
    assert_eq!(format_cents(7), "$0.07");
    assert_eq!(format_cents(-5), "-$0.05");
    assert_eq!(format_cents(123_456_789), "$1234567.89");
}

#[test]
fn renders_unchanged() {
    let lines = vec![Line::new("coffee", 350), Line::new("refund", -100)];
    assert_eq!(render_receipt(&lines), "coffee $3.50, refund -$1.00 = $2.50");
    let inv = render_invoice("Bo", &[Line::new("fee", 99), Line::new("credit", -1000)]);
    let want = format!(
        "INVOICE for Bo\n  {:<20}{:>12}\n  {:<20}{:>12}\n  {:<20}{:>12}\n",
        "fee", "$0.99", "credit", "-$10.00", "TOTAL", "-$9.01"
    );
    assert_eq!(inv, want);
}

#[test]
fn no_more_duplicates() {
    for src in [include_str!("../src/invoice.rs"), include_str!("../src/receipt.rs")] {
        assert!(!src.contains("% 100"), "cents formatting still duplicated");
        assert!(src.contains("format_cents"));
    }
}

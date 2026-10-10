use task::invoice::{Invoice, Line};
use task::pricing::{self, VAT_RATE};

#[test]
fn constant() {
    assert_eq!(VAT_RATE, 0.2);
}

#[test]
fn pricing_unchanged() {
    assert_eq!(pricing::gross(1000), 1200);
    assert_eq!(pricing::vat_of(999), 200);
    assert_eq!(pricing::net_from_gross(1200), 1000);
}

#[test]
fn invoice_unchanged() {
    let inv = Invoice {
        lines: vec![
            Line { item: "pen".into(), qty: 3, unit_cents: 150 },
            Line { item: "pad".into(), qty: 1, unit_cents: 499 },
        ],
    };
    assert_eq!(inv.vat_total(), 190);
    assert_eq!(inv.footer(), "VAT 20%: 190 cents, total 1139 cents");
}

#[test]
fn no_literal_left() {
    for src in [include_str!("../src/pricing.rs"), include_str!("../src/invoice.rs")] {
        let uses = src.lines().filter(|l| l.contains("0.2") && !l.contains("const")).count();
        assert_eq!(uses, 0, "0.2 still hard-coded");
    }
}

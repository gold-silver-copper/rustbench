use task::{Offer, cheapest};

#[test]
fn picks_lowest_price() {
    let offers = [
        Offer {
            supplier: "acme",
            cents: 1299,
        },
        Offer {
            supplier: "bolt",
            cents: 999,
        },
        Offer {
            supplier: "cog",
            cents: 1500,
        },
    ];
    assert_eq!(cheapest(&offers), Some("bolt"));
}

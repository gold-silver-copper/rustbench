use task::{Offer, cheapest};

#[test]
fn ties_and_single() {
    let tie = [
        Offer {
            supplier: "x",
            cents: 500,
        },
        Offer {
            supplier: "y",
            cents: 500,
        },
    ];
    assert_eq!(cheapest(&tie), Some("x"));
    assert_eq!(
        cheapest(&[Offer {
            supplier: "solo",
            cents: 7
        }]),
        Some("solo")
    );
    assert_eq!(cheapest(&[]), None);
}

#[test]
fn last_is_cheapest() {
    let offers = [
        Offer {
            supplier: "a",
            cents: 300,
        },
        Offer {
            supplier: "b",
            cents: 200,
        },
        Offer {
            supplier: "c",
            cents: 100,
        },
    ];
    assert_eq!(cheapest(&offers), Some("c"));
}

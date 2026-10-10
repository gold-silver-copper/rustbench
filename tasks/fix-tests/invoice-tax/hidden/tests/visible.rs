use task::{Line, total};

#[test]
fn total_includes_tax() {
    let lines = [
        Line {
            unit_cents: 1000,
            qty: 2,
        },
        Line {
            unit_cents: 500,
            qty: 1,
        },
    ];
    assert_eq!(total(&lines, 2000), 3000);
}

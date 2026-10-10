use task::{Line, tax, total};

#[test]
fn other_rates() {
    let lines = [Line {
        unit_cents: 2500,
        qty: 4,
    }];
    assert_eq!(total(&lines, 825), 10_825);
    assert_eq!(total(&lines, 0), 10_000);
    assert_eq!(total(&[], 2000), 0);
}

#[test]
fn tax_rounds_down() {
    assert_eq!(tax(999, 1000), 99);
    assert_eq!(
        total(
            &[Line {
                unit_cents: 999,
                qty: 1
            }],
            1000
        ),
        1098
    );
}

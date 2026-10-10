use task::{Day, high, warmest_day};

#[test]
fn highs() {
    assert_eq!(
        high(&Day {
            name: "a",
            readings: vec![12, -4, 30]
        }),
        Some(30)
    );
    assert_eq!(
        high(&Day {
            name: "b",
            readings: vec![-7]
        }),
        Some(-7)
    );
    assert_eq!(
        high(&Day {
            name: "c",
            readings: vec![]
        }),
        None
    );
}

#[test]
fn warmest_of_a_cold_week() {
    let days = [
        Day {
            name: "tue",
            readings: vec![-120, -95],
        },
        Day {
            name: "wed",
            readings: vec![-40, -60],
        },
        Day {
            name: "thu",
            readings: vec![],
        },
        Day {
            name: "fri",
            readings: vec![-200],
        },
    ];
    assert_eq!(warmest_day(&days), Some("wed"));
}

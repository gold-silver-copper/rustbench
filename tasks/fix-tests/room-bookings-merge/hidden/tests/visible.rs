use task::{Block, busy};

#[test]
fn long_booking_swallows_short_one() {
    let bookings = [
        Block {
            start: 540,
            end: 720,
        },
        Block {
            start: 600,
            end: 630,
        },
    ];
    assert_eq!(
        busy(&bookings),
        vec![Block {
            start: 540,
            end: 720
        }]
    );
}

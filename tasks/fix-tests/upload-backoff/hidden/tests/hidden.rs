use task::{delays, total_wait_ms};

#[test]
fn schedules() {
    assert_eq!(delays(0, 100, 10_000), Vec::<u64>::new());
    assert_eq!(delays(1, 250, 10_000), vec![250]);
    assert_eq!(delays(5, 500, 3000), vec![500, 1000, 2000, 3000, 3000]);
}

#[test]
fn totals() {
    assert_eq!(total_wait_ms(4, 100, 10_000), 1500);
    assert_eq!(total_wait_ms(0, 100, 10_000), 0);
}

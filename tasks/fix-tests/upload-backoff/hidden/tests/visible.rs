use task::delays;

#[test]
fn three_retries_three_delays() {
    assert_eq!(delays(3, 100, 10_000), vec![100, 200, 400]);
}

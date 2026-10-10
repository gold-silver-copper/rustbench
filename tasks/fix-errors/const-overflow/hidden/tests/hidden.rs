#[test]
fn converts() {
    assert_eq!(task::days_to_ms(1), 86_400_000);
    assert_eq!(task::days_to_ms(7), 604_800_000);
    assert_eq!(task::MS_PER_DAY as u64, 86_400_000);
}

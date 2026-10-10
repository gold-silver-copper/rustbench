use task::parse_secs;

#[test]
fn hours_and_minutes_add_up() {
    assert_eq!(parse_secs("1h30m"), Ok(5400));
}

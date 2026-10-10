use task::moving_average;

#[test]
fn averages_every_window() {
    assert_eq!(moving_average(&[2, 4, 6, 8], 2), vec![3.0, 5.0, 7.0]);
}

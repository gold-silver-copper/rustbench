use task::moving_average;

#[test]
fn window_as_long_as_the_data() {
    assert_eq!(moving_average(&[1000, 3000, 2000], 3), vec![2000.0]);
}

#[test]
fn window_of_one_and_edges() {
    assert_eq!(moving_average(&[5, 7], 1), vec![5.0, 7.0]);
    assert_eq!(moving_average(&[5, 7], 3), Vec::<f64>::new());
    assert_eq!(moving_average(&[5, 7], 0), Vec::<f64>::new());
    assert_eq!(moving_average(&[], 1), Vec::<f64>::new());
}

#[test]
fn week_of_steps() {
    let week = [4000, 6000, 8000, 10000, 2000, 0, 6000];
    assert_eq!(
        moving_average(&week, 2),
        vec![5000.0, 7000.0, 9000.0, 6000.0, 1000.0, 3000.0]
    );
}

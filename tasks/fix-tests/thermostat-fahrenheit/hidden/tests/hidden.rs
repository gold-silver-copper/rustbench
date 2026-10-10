use task::{c_to_f, f_to_c};

#[test]
fn more_temperatures() {
    assert_eq!(c_to_f(0), 32);
    assert_eq!(c_to_f(-40), -40);
    assert_eq!(c_to_f(37), 98);
    assert_eq!(c_to_f(21), 69);
}

#[test]
fn back_again() {
    assert_eq!(f_to_c(212), 100);
    assert_eq!(f_to_c(-40), -40);
}

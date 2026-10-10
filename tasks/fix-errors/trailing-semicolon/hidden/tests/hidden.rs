#[test]
fn interest() {
    let l = task::Loan { principal: 1000.0, yearly_rate: 0.05, years: 3 };
    assert!((l.simple_interest() - 150.0).abs() < 1e-9);
    assert!((l.total_owed() - 1150.0).abs() < 1e-9);
}

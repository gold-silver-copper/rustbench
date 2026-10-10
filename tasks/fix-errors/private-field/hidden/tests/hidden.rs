#[test]
fn statement() {
    let mut a = task::account::Account::open("Ana");
    a.deposit(1000);
    a.deposit(500);
    assert_eq!(task::statement(&a), "Ana: 1500 cents");
    assert_eq!(a.balance(), 1500);
}

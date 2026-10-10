use task::Ledger;

#[test]
fn pay_moves_money_to_payee() {
    let mut l = Ledger::default();
    l.open("alice", 1000);
    l.open("bob", 0);
    assert_eq!(l.pay("alice", "bob", 250), Ok(()));
    assert_eq!(l.balance("alice"), Some(750));
    assert_eq!(l.balance("bob"), Some(250));
}

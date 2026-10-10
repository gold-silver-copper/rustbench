use task::{Ledger, TransferError};

#[test]
fn payer_must_have_the_money() {
    let mut l = Ledger::default();
    l.open("carol", 100);
    l.open("dave", 5000);
    assert_eq!(
        l.pay("carol", "dave", 200),
        Err(TransferError::InsufficientFunds)
    );
    assert_eq!(l.balance("carol"), Some(100));
    assert_eq!(l.balance("dave"), Some(5000));
    assert_eq!(l.pay("dave", "carol", 200), Ok(()));
    assert_eq!(l.balance("carol"), Some(300));
    assert_eq!(l.balance("dave"), Some(4800));
}

#[test]
fn unknown_accounts() {
    let mut l = Ledger::default();
    l.open("erin", 100);
    assert_eq!(
        l.pay("erin", "nobody", 10),
        Err(TransferError::UnknownAccount)
    );
    assert_eq!(
        l.pay("nobody", "erin", 10),
        Err(TransferError::UnknownAccount)
    );
    assert_eq!(l.balance("erin"), Some(100));
}

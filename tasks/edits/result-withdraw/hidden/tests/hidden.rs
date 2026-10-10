use task::{Account, BankError, ledger};

fn funded(id: u32, amount: u64) -> Account {
    let mut a = Account::open(id, "x");
    a.deposit(amount);
    a
}

#[test]
fn withdraw_ok() {
    let mut a = funded(1, 100);
    assert_eq!(a.withdraw(40), Ok(()));
    assert_eq!(a.withdraw(60), Ok(()));
    assert_eq!(a.balance, 0);
}

#[test]
fn withdraw_too_much_is_an_error() {
    let mut a = funded(1, 50);
    assert_eq!(
        a.withdraw(80),
        Err(BankError::InsufficientFunds { needed: 80, available: 50 })
    );
    assert_eq!(a.balance, 50);
}

#[test]
fn transfer_and_fee_unchanged() {
    let mut a = funded(1, 100);
    let mut b = funded(2, 0);
    assert_eq!(ledger::transfer(&mut a, &mut b, 70), Ok(()));
    assert_eq!((a.balance, b.balance), (30, 70));
    assert_eq!(
        ledger::transfer(&mut a, &mut b, 31),
        Err(BankError::InsufficientFunds { needed: 31, available: 30 })
    );
    assert_eq!((a.balance, b.balance), (30, 70));
    let mut c = a.clone();
    assert_eq!(ledger::transfer(&mut a, &mut c, 1), Err(BankError::SameAccount));
    ledger::charge_fee(&mut a, 50);
    assert_eq!(a.balance, 0);
    ledger::charge_fee(&mut b, 5);
    assert_eq!(b.balance, 65);
}

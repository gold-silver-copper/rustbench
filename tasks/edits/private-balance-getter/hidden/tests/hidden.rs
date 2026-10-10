use task::Wallet;
use task::report::{richest, summary};

#[test]
fn getter() {
    let mut w = Wallet::new("ana");
    assert_eq!(w.balance(), 0);
    w.deposit(1250);
    assert!(w.spend(300));
    assert!(!w.spend(5000));
    assert_eq!(w.balance(), 950);
    assert_eq!(summary(&w), "ana: 9.50");
}

#[test]
fn richest_still_works() {
    let mut a = Wallet::new("a");
    a.deposit(10);
    let mut b = Wallet::new("b");
    b.deposit(20);
    let all = [a, b];
    assert_eq!(richest(&all).map(|w| w.owner.as_str()), Some("b"));
}

#[test]
fn field_is_private() {
    let src = include_str!("../src/wallet.rs");
    assert!(!src.contains("pub balance:"));
    assert!(src.contains("pub owner"));
}

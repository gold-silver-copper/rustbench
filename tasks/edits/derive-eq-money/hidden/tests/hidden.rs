use task::{Currency, Money};

fn requires_eq<T: Eq>() {}

#[test]
fn money_compares() {
    requires_eq::<Money>();
    requires_eq::<Currency>();
    let a = Money::new(1_000, Currency::Gbp);
    assert_eq!(a.checked_add(Money::new(250, Currency::Gbp)), Some(Money::new(1_250, Currency::Gbp)));
    assert_eq!(a.checked_add(Money::new(250, Currency::Usd)), None);
    assert_ne!(Money::new(5, Currency::Eur), Money::new(5, Currency::Usd));
    assert_ne!(Currency::Eur, Currency::Gbp);
}

#[test]
fn split_unchanged() {
    let shares = task::split::split_evenly(Money::new(1_001, Currency::Eur), 3);
    assert_eq!(
        shares,
        vec![
            Money::new(334, Currency::Eur),
            Money::new(334, Currency::Eur),
            Money::new(333, Currency::Eur),
        ]
    );
    assert_eq!(Money::new(-1_250, Currency::Usd).display(), "-12.50 USD");
}

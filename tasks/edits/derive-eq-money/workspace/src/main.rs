use task::{Currency, Money};

fn main() {
    let dinner = Money::new(8_750, Currency::Eur);
    let drinks = Money::new(2_100, Currency::Eur);
    if let Some(total) = dinner.checked_add(drinks) {
        for (i, share) in task::split::split_evenly(total, 3).iter().enumerate() {
            println!("person {}: {}", i + 1, share.display());
        }
    }
    println!("{:?}", dinner.checked_add(Money::new(100, Currency::Usd)).map(Money::display));
}

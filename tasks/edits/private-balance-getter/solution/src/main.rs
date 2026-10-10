use task::Wallet;
use task::report::{richest, summary};

fn main() {
    let mut a = Wallet::new("ana");
    a.deposit(1250);
    a.spend(300);
    let mut b = Wallet::new("ben");
    b.deposit(400);
    if b.balance() < 500 {
        println!("{} is running low", b.owner);
    }
    let all = [a, b];
    if let Some(w) = richest(&all) {
        println!("{}", summary(w));
    }
}

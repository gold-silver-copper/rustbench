use task::{Account, ledger};

fn main() {
    let mut alice = Account::open(1, "alice");
    let mut bob = Account::open(2, "bob");
    alice.deposit(500);
    if let Err(e) = alice.withdraw(120) {
        println!("withdraw failed: {e}");
    }
    if let Err(e) = ledger::transfer(&mut alice, &mut bob, 300) {
        println!("transfer failed: {e}");
    }
    ledger::charge_fee(&mut bob, 5);
    println!("alice {} bob {}", alice.balance, bob.balance);
}

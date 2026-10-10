#[derive(Debug)]
pub struct Wallet {
    pub owner: String,
    balance: i64,
}

impl Wallet {
    pub fn new(owner: &str) -> Self {
        Wallet {
            owner: owner.to_string(),
            balance: 0,
        }
    }

    pub fn balance(&self) -> i64 {
        self.balance
    }

    pub fn deposit(&mut self, cents: i64) {
        self.balance += cents;
    }

    /// Spend `cents` if there is enough money; returns whether it happened.
    pub fn spend(&mut self, cents: i64) -> bool {
        if cents > self.balance {
            return false;
        }
        self.balance -= cents;
        true
    }
}

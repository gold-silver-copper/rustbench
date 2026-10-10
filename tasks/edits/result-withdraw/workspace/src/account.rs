#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: u32,
    pub owner: String,
    pub balance: u64,
}

impl Account {
    pub fn open(id: u32, owner: &str) -> Self {
        Account {
            id,
            owner: owner.to_string(),
            balance: 0,
        }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    /// Take `amount` out of the account.
    ///
    /// Panics if the balance is too low.
    pub fn withdraw(&mut self, amount: u64) {
        if amount > self.balance {
            panic!("insufficient funds in account {}", self.id);
        }
        self.balance -= amount;
    }
}

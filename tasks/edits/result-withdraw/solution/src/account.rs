use crate::error::BankError;

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
    pub fn withdraw(&mut self, amount: u64) -> Result<(), BankError> {
        if amount > self.balance {
            return Err(BankError::InsufficientFunds {
                needed: amount,
                available: self.balance,
            });
        }
        self.balance -= amount;
        Ok(())
    }
}

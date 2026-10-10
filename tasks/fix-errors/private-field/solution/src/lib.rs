pub mod account {
    pub struct Account {
        owner: String,
        balance_cents: i64,
    }

    impl Account {
        pub fn open(owner: &str) -> Account {
            Account { owner: owner.to_string(), balance_cents: 0 }
        }

        pub fn deposit(&mut self, cents: i64) {
            self.balance_cents += cents;
        }

        pub fn balance(&self) -> i64 {
            self.balance_cents
        }

        pub fn owner(&self) -> &str {
            &self.owner
        }
    }
}

/// One-line statement, e.g. "Ana: 1500 cents".
pub fn statement(acct: &account::Account) -> String {
    format!("{}: {} cents", acct.owner(), acct.balance())
}

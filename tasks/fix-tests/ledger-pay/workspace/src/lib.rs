//! A tiny ledger of account balances, in cents.

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum TransferError {
    UnknownAccount,
    InsufficientFunds,
}

#[derive(Default)]
pub struct Ledger {
    balances: HashMap<String, i64>,
}

impl Ledger {
    pub fn open(&mut self, name: &str, cents: i64) {
        self.balances.insert(name.to_string(), cents);
    }

    pub fn balance(&self, name: &str) -> Option<i64> {
        self.balances.get(name).copied()
    }

    fn move_cents(&mut self, from: &str, to: &str, cents: i64) -> Result<(), TransferError> {
        let available = self.balance(from).ok_or(TransferError::UnknownAccount)?;
        if !self.balances.contains_key(to) {
            return Err(TransferError::UnknownAccount);
        }
        if available < cents {
            return Err(TransferError::InsufficientFunds);
        }
        *self.balances.entry(from.to_string()).or_default() -= cents;
        *self.balances.entry(to.to_string()).or_default() += cents;
        Ok(())
    }

    /// Pays `cents` from `payer` to `payee`.
    pub fn pay(&mut self, payer: &str, payee: &str, cents: i64) -> Result<(), TransferError> {
        self.move_cents(payee, payer, cents)
    }
}

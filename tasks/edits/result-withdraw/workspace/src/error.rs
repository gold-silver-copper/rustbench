use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BankError {
    InsufficientFunds { needed: u64, available: u64 },
    SameAccount,
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BankError::InsufficientFunds { needed, available } => {
                write!(f, "insufficient funds: needed {needed}, available {available}")
            }
            BankError::SameAccount => write!(f, "cannot transfer to the same account"),
        }
    }
}

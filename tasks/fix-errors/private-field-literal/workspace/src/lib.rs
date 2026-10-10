mod wallet;

pub use wallet::Wallet;

/// Every new player starts with 100 coins.
pub fn starter_wallet(owner: &str) -> Wallet {
    Wallet { owner: owner.to_string(), coins: 100 }
}

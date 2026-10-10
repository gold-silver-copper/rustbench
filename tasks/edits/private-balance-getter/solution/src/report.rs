use crate::wallet::Wallet;

pub fn summary(w: &Wallet) -> String {
    format!(
        "{}: {}.{:02}",
        w.owner,
        w.balance() / 100,
        w.balance() % 100
    )
}

pub fn richest(wallets: &[Wallet]) -> Option<&Wallet> {
    wallets.iter().max_by_key(|w| w.balance())
}

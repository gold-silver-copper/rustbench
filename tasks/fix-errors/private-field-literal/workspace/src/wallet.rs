pub struct Wallet {
    owner: String,
    coins: u32,
}

impl Wallet {
    pub fn new(owner: &str, coins: u32) -> Wallet {
        Wallet { owner: owner.to_string(), coins }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn coins(&self) -> u32 {
        self.coins
    }

    /// Spends coins if there are enough; returns whether it worked.
    pub fn spend(&mut self, amount: u32) -> bool {
        if amount > self.coins {
            return false;
        }
        self.coins -= amount;
        true
    }
}

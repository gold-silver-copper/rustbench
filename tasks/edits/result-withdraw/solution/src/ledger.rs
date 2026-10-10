use crate::account::Account;
use crate::error::BankError;

/// Move `amount` from `from` to `to`.
pub fn transfer(from: &mut Account, to: &mut Account, amount: u64) -> Result<(), BankError> {
    if from.id == to.id {
        return Err(BankError::SameAccount);
    }
    from.withdraw(amount)?;
    to.deposit(amount);
    Ok(())
}

/// Charge a monthly fee; accounts that can't cover it are emptied instead.
pub fn charge_fee(acct: &mut Account, fee: u64) {
    let take = fee.min(acct.balance);
    // `take` never exceeds the balance, so this cannot fail.
    let _ = acct.withdraw(take);
}

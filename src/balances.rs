use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance!")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

pub enum Call<T: Config> {
    Transfer{ to: T::AccountId, amount: T::Balance },
}

impl <T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(&caller, &to, amount)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{balances::Pallet, system};

    struct TestConfig;

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances: Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);

        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances: Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&"alice".to_string(), 100);
        let _ = balances.transfer(&"alice".to_string(), &"jopa".to_string(), 25);

        assert_eq!(balances.balance(&"jopa".to_string()), 25);
        assert_eq!(balances.balance(&"alice".to_string()), 75);
    }

    #[test]
    fn transfer_balance_overflow() {
        let mut balances: Pallet<TestConfig> = super::Pallet::new();

        balances.set_balance(&"alice".to_string(), u128::MAX);
        balances.set_balance(&"bob".to_string(), 1);
        let result = balances.transfer(&"bob".to_string(), &"alice".to_string(),  1);
    
        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(balances.balance(&"alice".to_string()), u128::MAX);
        assert_eq!(balances.balance(&"bob".to_string()), 1)
    }
}
use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
} 

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => { Err("Claim already exists!") },
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.get_claim(&claim).ok_or("Claim does not exist!")?; 

        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim!");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::result;

    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();
        let _ = poe.create_claim("alice", "tvorojok");

        assert_eq!(poe.get_claim(&"tvorojok"), Some(&"alice"));

        let result = poe.revoke_claim("bob", "my_document");
        assert_eq!(result, Err("Claim does not exist!"));

        let result = poe.create_claim("bob", "tvorojok");
        assert_eq!(result, Err("Claim already exists!"));

        let result = poe.revoke_claim("bob", "tvorojok");
        assert_eq!(result, Err("Caller is not the owner of the claim!"));

        let result = poe.revoke_claim("alice", "tvorojok");
        assert_eq!(result, Ok(()));
        assert_eq!(poe.get_claim(&"tvorojok"), None);
    }
}
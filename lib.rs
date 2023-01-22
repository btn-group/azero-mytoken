#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod mytoken {
    use ink_storage::{traits::SpreadAllocate, Mapping};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Mytoken {
        total_supply: u32,
        balances: Mapping<AccountId, u32>,
    }

    use ink_lang::utils::initialize_contract;
    impl Mytoken {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new_token(supply: u32) -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.balances.insert(&caller, &supply);
                contract.total_supply = supply;
            })
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let mytoken = Mytoken::default();
            assert_eq!(mytoken.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut mytoken = Mytoken::new(false);
            assert_eq!(mytoken.get(), false);
            mytoken.flip();
            assert_eq!(mytoken.get(), true);
        }
    }
}

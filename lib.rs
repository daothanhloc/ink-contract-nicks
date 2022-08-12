#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod nicks {

    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Nicks {
        /// Stores a single `bool` value on the storage.
        nick_names: ink_storage::Mapping<AccountId, String>,
    }

    impl Nicks {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                let nick_name = String::from("");
                contract.nick_names.insert(&caller, &nick_name);
            })
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn set_nick_name(&mut self, nick_name: String) {
            let caller = Self::env().caller();
            self.nick_names.insert(&caller, &nick_name);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_mine(&self) -> String {
            self.nick_names
                .get(&Self::env().caller())
                .unwrap_or_default()
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
            let nicks = Nicks::default();
            assert_eq!(nicks.get_mine(), String::from(""));
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut nicks = Nicks::new();
            assert_eq!(nicks.get_mine(), String::from(""));
            nicks.set_nick_name(String::from("LocDT"));
            assert_eq!(nicks.get_mine(), String::from("LocDT"));
        }
    }
}

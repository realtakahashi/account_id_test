#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod account_id_test {
    use ink::prelude::{ vec, vec::Vec };
    use ink::prelude::string::{ String, ToString };
    use scale::{Decode, Encode};
//    use sp_runtime::AccountId32;
    
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct AccountIdTest {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl AccountIdTest {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn convert_accountid_to_string(&self, account_id: AccountId) -> Vec<u8> {
            let tmp :Vec<u8> = account_id.encode();
            tmp
            //String::from_utf8(tmp).unwrap()
        }

        #[ink(message)]
        pub fn convert_accountid_to_string2(&self, account_id: AccountId) -> String {
            let tmp :Vec<u8> = account_id.encode();
            tmp
            //String::from_utf8(tmp).unwrap()
        }

        #[ink(message)]
        pub fn convert_string_to_accountid(&self, account_str: String) -> Option<AccountId> {
            let mut output = vec![0xFF; 35];
            match bs58::decode(account_str).into(&mut output) {
                Ok(_) => (),
                Err(_) => return None,
            };
            let cut_address_vec: Vec<_> = output.drain(1..33).collect();
            let mut array = [0; 32];
            let bytes = &cut_address_vec[..array.len()];
            array.copy_from_slice(bytes);
            let account_id: AccountId = array.into();
            Some(account_id)
        }
        
        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let account_id_test = AccountIdTest::default();
            assert_eq!(account_id_test.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut account_id_test = AccountIdTest::new(false);
            assert_eq!(account_id_test.get(), false);
            account_id_test.flip();
            assert_eq!(account_id_test.get(), true);
        }
    }
}

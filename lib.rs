#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod account_id_test {
    use ink::prelude::{ vec, vec::Vec };
    use ink::prelude::string::{ String, ToString };
    use ink::prelude::format;
    use scale::{Decode, Encode};
    use core::str::FromStr;
    // use sp_core::crypto::Ss58Codec;
    // use sp_core::H256;
//    use sp_runtime::AccountId32;
   
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct AccountIdTest {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    // use ink_env::AccountId; 
    // use sp_core::H256; 
    // use core::convert::From;  
    // impl From<AccountId> for H256 {
    //     fn from(account_id: AccountId) -> Self {
    //          let mut h256 = H256::default();
    //          h256.as_mut().copy_from_slice(&account_id[..]);         
    //          h256     
    //     } 
    // }

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
        pub fn convert_accountid_to_hexstring(&self, account_id: AccountId) -> String {
            hex::encode(account_id)
        }

        #[ink(message)]
        pub fn convert_hexstring_to_accountid(&self, hex_string:String) -> Option<AccountId> {
            match hex::decode(hex_string) {
                Ok(value) => {
                    let mut array = [0; 32];
                    let bytes = &value[..array.len()];
                    array.copy_from_slice(bytes);
                    let account_id: AccountId = array.into();
                    return Some(account_id);
                },
                Err(_) => None,
            }
        }

        // #[ink(message)]
        // pub fn convert_accountid_to_string2(&self, account_id: AccountId) -> String {
        //     // let mut output = vec![0; 32];
        //     //let mut output:Vec<u8> = Vec::new();
        //     let mut output = [0];
        //     // let mut output: String = "Hello, world!".to_string();
        //     // let mut output2 = output.into_bytes();
        //     match bs58::encode(account_id).into(&mut output){
        //         Ok(_) => (),
        //         Err(_) => return "".to_string(),
        //     };
        //     match String::from_utf8(output) {
        //         Ok(value) => value,
        //         Err(_) => "".to_string(),
        //     }
        // }

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

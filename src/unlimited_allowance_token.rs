use std::string::String;
use std::collections::HashMap;
use near_bindgen::{near_bindgen, ENV, MockedEnvironment};
use serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(Default, Serialize, Deserialize)]
pub struct UnlimitedAllowanceToken {
     balances: HashMap<Vec<u8>, u64>,
     pub creator: Vec<u8>,
     pub name: String,
     pub max_supply: u64,
     pub initialized: bool,
}

#[near_bindgen]
impl UnlimitedAllowanceToken {
     pub fn init(&mut self, name: String, max_supply: u64) {
          if self.initialized == false {
               let account_id = ENV.originator_id();
               self.creator = account_id;
               self.name = name;
               self.max_supply = max_supply;
               self.balances.insert(self.creator.to_owned(), max_supply);
               self.initialized = true
          }
     }

     pub fn get_balance_of(&self, owner: String) -> &u64 {
          let owner = owner.into_bytes();
          let balance_option =  self.balances.get(&owner);
          
          match balance_option {
               Some (balance) => return balance,
               None => return &0,
          }
     }

     pub fn transfer(&mut self, to: String, amount: u64) -> bool{
          let from_id: Vec<u8> = ENV.originator_id();
          let from_balance_optional: Option<&u64> = self.balances.get(&from_id);
          let from_balance: &u64 = check_optional(from_balance_optional);
          if from_balance < &amount {
               return false;
          }
          let to_id: Vec<u8> = to.into_bytes();
          let to_balance_optional: Option<&u64> = self.balances.get(&to_id);
          let to_balance: &u64 = check_optional(to_balance_optional);

          let new_from_balance = from_balance - amount;
          let new_to_balance = to_balance + amount;
          self.balances.insert(from_id, new_from_balance);
          self.balances.insert(to_id, new_to_balance);

          return true;
     }
}

fn check_optional(optional: Option<&u64>) -> &u64 {
     match optional {
          Some (data) => return data,
          None => return &0,
     } 
}


#[test]
fn setupAndTransferToken() {
     ENV.set(Box::new(MockedEnvironment::new()));
     let account_1_id = "alice";
     let account_2_id = "bob";

     ENV.as_mock().set_originator_id(account_1_id.as_bytes().to_vec());

     let mut contract = UnlimitedAllowanceToken::default();
     let max_supply: u64 = 1000000;
     let name: String = "test token".to_string();
     contract.init(name.to_owned(), max_supply);
     assert_eq!(contract.creator, account_1_id.as_bytes().to_vec());
     assert_eq!(contract.max_supply, max_supply);
     assert_eq!(contract.initialized, true);
     assert_eq!(contract.name, name);

     let mut account_1_balance: &u64 = contract.get_balance_of(account_1_id.to_owned());
     assert_eq!(account_1_balance, &max_supply);

     let transfered = contract.transfer(account_2_id.to_owned(), 100);

     account_1_balance = contract.get_balance_of(account_1_id.to_owned());
     let mut expected_supply: u64 = max_supply - 100;
     assert_eq!(account_1_balance, &expected_supply);

     let account_2_balance = contract.get_balance_of(account_2_id.to_owned());
     expected_supply = 100;
     assert_eq!(account_2_balance, &expected_supply);
}
use std::string::String;
use std::collections::HashMap;
use near_bindgen::{near_bindgen, ENV, MockedEnvironment};
use serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(Default, Serialize, Deserialize)]
pub struct FungibleToken {
     balances: HashMap<Vec<u8>, u64>,
     pub creator: Vec<u8>,
     pub name: String,
     pub max_supply: u64,
     pub initialized: bool,
}

#[near_bindgen]
impl FungibleToken {
     pub fn init(&mut self, name: String, max_supply: u64) {
          if self.initialized == false {
               let account_id = ENV.originator_id();
               self.creator = account_id;
               self.name = name;
               self.max_supply = max_supply;
               self.balances.insert(self.creator.clone(), max_supply);
               self.initialized = true
          }
     }

     pub fn get_balance_of(&self, owner: String) -> &u64 {
          let owner = owner.into_bytes();
          let balance =  self.balances.get(&owner).unwrap_or_default();
     }

     pub fn transfer(&mut self, to: String, amount: u64) -> bool{
          let from_id = ENV.originator_id();
          let from_balance = self.balances.get(&from_id).unwrap_or_default();
          let to_id= to.into_bytes();
          let to_balance= self.balances.get(&to_id).unwrap_or_default();
          self.balances.insert(from_id, from_balance - amount);
          self.balances.insert(to_id, to_balance + amount);
          return true;
     }
}



#[test]
fn setupAndTransferToken() {
     ENV.set(Box::new(MockedEnvironment::new()));
     let account_1_id = "alice";
     let account_2_id = "bob";

     ENV.as_mock().set_originator_id(account_1_id.as_bytes().to_vec());

     let mut contract = FungibleToken::default();
     let max_supply = 1000000;
     let name = "test token".to_string();
     contract.init(name.clone(), max_supply);
     assert_eq!(contract.creator, account_1_id.as_bytes().to_vec());
     assert_eq!(contract.max_supply, max_supply);
     assert_eq!(contract.initialized, true);
     assert_eq!(contract.name, name);

     let mut account_1_balance = contract.get_balance_of(account_1_id.to_string());
     assert_eq!(account_1_balance, &max_supply);

     let transfered = contract.transfer(account_2_id.to_string(), 100);

     account_1_balance = contract.get_balance_of(account_1_id.to_string());
     let mut expected_supply: u64 = max_supply - 100;
     assert_eq!(account_1_balance, &expected_supply);

     let account_2_balance = contract.get_balance_of(account_2_id.to_string());
     expected_supply = 100;
     assert_eq!(account_2_balance, &expected_supply);
}
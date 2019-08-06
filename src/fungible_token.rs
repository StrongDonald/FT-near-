use std::string::String;
use std::collections::HashMap;
use near_bindgen::{near_bindgen, ENV, MockedEnvironment};
use serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(Serialize, Deserialize)]
pub struct FungibleToken {
     balances: HashMap<Vec<u8>, u64>,
     allowances: HashMap<Vec<u8>, u64>,
     pub creator: Vec<u8>,
     pub name: String,
     pub max_supply: u64,
}

#[near_bindgen]
impl FungibleToken {
     pub fn transfer(&mut self, to: String, amount: u64) -> bool{
          let from_id = ENV.originator_id();
          let from_balance = self.balances.get(&from_id).unwrap_or(&0);
          if from_balance < &amount {return false};
          let to_id= to.into_bytes();
          let to_balance= self.balances.get(&to_id).unwrap_or(&0);

          let new_from_balance = from_balance - amount;
          let new_to_balance = to_balance + amount;

          self.balances.insert(from_id, new_from_balance);
          self.balances.insert(to_id, new_to_balance);
          return true;
     }

     pub fn get_balance_of(&self, owner: String) -> &u64 {
          let owner = owner.into_bytes();
          let balance = self.balances.get(&owner).unwrap_or(&0);
          return balance;
     }
}


impl Default for FungibleToken {
    fn default() -> Self {
          let mut balances = HashMap::new();
          let max_supply = 1000000000;
          balances.insert(ENV.originator_id(), max_supply);
          Self { 
               balances: balances,
               allowances: HashMap::new(),
               creator: ENV.originator_id(),
               name: String::from("FungToken"),
               max_supply: max_supply,
          }
    }
}

#[test]
fn setupAndTransferToken() {
     ENV.set(Box::new(MockedEnvironment::new()));
     let account_1_id = "alice";
     let account_2_id = "bob";

     ENV.as_mock().set_originator_id(account_1_id.as_bytes().to_vec());
     let name = String::from("FungToken");
     let max_supply = 1000000000;
     let mut contract = FungibleToken::default();
     assert_eq!(contract.creator, account_1_id.as_bytes().to_vec());
     assert_eq!(contract.max_supply, max_supply);
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
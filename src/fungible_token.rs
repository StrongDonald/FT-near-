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

     pub fn set_allowance(&mut self, spender: &String, allowance: u64) -> bool {
          let from_id = ENV.originator_id();
          let spender_id = spender.to_string().into_bytes().to_vec();
          let flat_id = [from_id, spender_id].concat();
          self.allowances.insert(flat_id, allowance);
          return true;
     }

     pub fn transfer(&mut self, to: &String, amount: u64) -> bool {
          let from_id = ENV.originator_id();
          let to_id= to.to_string().into_bytes().to_vec();
          let from_balance = self.balances.get(&from_id).unwrap_or(&0);
          let to_balance= self.balances.get(&to_id).unwrap_or(&0);
          
          if from_balance < &amount {return false};

          let new_from_balance = from_balance - amount;
          let new_to_balance = to_balance + amount;

          self.balances.insert(from_id, new_from_balance);
          self.balances.insert(to_id, new_to_balance);
          return true;
     }

     pub fn transfer_from(&mut self, from: &String, to: &String, amount: u64) -> bool {
          let from_id = from.to_string().into_bytes().to_vec();
          let spender_id = ENV.originator_id();
          let to_id = to.to_string().into_bytes().to_vec();
          let flat_id = [from_id.to_vec(), spender_id].concat();
          let from_balance = self.get_balance_of(from);
          let to_balance = self.get_balance_of(&to);
          let spender_allowance = self.allowances.get(&flat_id).unwrap_or(&0);

          if from_balance < &amount {return false;} 
          else if spender_allowance < &amount {return false;}

          let new_allowance = spender_allowance - amount;
          let new_from_balance = from_balance - amount;
          let new_to_balance = to_balance + amount;

          self.allowances.insert(flat_id, new_allowance);
          self.balances.insert(from_id, new_from_balance);
          self.balances.insert(to_id, new_to_balance);

          return true;
     }

     pub fn get_allowance_of(&self, owner: &String, spender: &String) -> &u64 {
          let from_id = owner.to_string().into_bytes().to_vec();
          let spender_id = spender.to_string().into_bytes().to_vec();
          let flat_id = [from_id, spender_id].concat();
          let allowance = self.allowances.get(&flat_id).unwrap_or(&0);
          return allowance;
     }

     pub fn get_balance_of(&self, owner: &String) -> &u64 {
          let owner = owner.to_string().into_bytes().to_vec();
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
fn setup_and_transfer_token() {
     ENV.set(Box::new(MockedEnvironment::new()));
     let account_1_id = String::from("alice");
     let account_2_id = String::from("bob");

     ENV.as_mock().set_originator_id(account_1_id.as_bytes().to_vec());
     let name = String::from("FungToken");
     let max_supply = 1000000000;
     let mut contract = FungibleToken::default();
     assert_eq!(contract.creator, account_1_id.as_bytes().to_vec());
     assert_eq!(contract.max_supply, max_supply);
     assert_eq!(contract.name, name);

     let mut account_1_balance = contract.get_balance_of(&account_1_id);
     assert_eq!(account_1_balance, &max_supply);

     let transfered = contract.transfer(&account_2_id, 100);

     account_1_balance = contract.get_balance_of(&account_1_id);
     let mut expected_balance = max_supply - 100;
     assert_eq!(account_1_balance, &expected_balance);

     let mut account_2_balance = contract.get_balance_of(&account_2_id);
     expected_balance = 100;
     assert_eq!(account_2_balance, &expected_balance);

     contract.set_allowance(&account_2_id, 100);

     let mut account_2_allowance = contract.get_allowance_of(&account_1_id, &account_2_id);
     assert_eq!(account_2_allowance, &100);

     ENV.as_mock().set_originator_id(account_2_id.as_bytes().to_vec());
     contract.transfer_from(&account_1_id, &account_2_id, 100);

     account_2_balance = contract.get_balance_of(&account_2_id);
     assert_eq!(account_2_balance, &200);

     account_2_allowance = contract.get_allowance_of(&account_1_id, &account_2_id);
     assert_eq!(account_2_allowance, &0);

     expected_balance = max_supply - 200;
     account_1_balance  = contract.get_balance_of(&account_1_id);
     assert_eq!(account_1_balance, &expected_balance);
}
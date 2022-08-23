use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{bs58, env, near_bindgen, PanicOnDefault, setup_alloc};
setup_alloc!();
pub mod utils;
pub mod view;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Snapshot {
  details: UnorderedMap<String, Info>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Info {
  block_height: u64,
  contract_address: String,
  hash: String,
}

#[near_bindgen]
impl Snapshot {
  #[init]
  pub fn new() -> Self {
    assert!(!env::state_exists(), "Already initialized");
    Self {
      details: UnorderedMap::new(b"d".to_vec()),
    }
  }

  pub fn set_snapshot(&mut self, contract_address: String) -> String {
    let hash = utils::get_hash(env::block_height(), contract_address.clone());
    let info = Info {
      block_height: env::block_height(),
      contract_address: contract_address.clone(),
      hash: hash.clone(),
    };
    self.details.insert(&info.hash, &info);
    hash.clone()
  }

  pub fn delete_snapshot(&mut self, hash: String) {
    self.details.remove(&hash);
  }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::test_utils::{get_logs, VMContextBuilder};
  use near_sdk::{testing_env, AccountId};

  #[test]
  fn set_snapshot() {
    // Basic set up for a unit test
    testing_env!(VMContextBuilder::new().build());
    let mut contract = Snapshot::new();
    let hash = contract.set_snapshot("jacktest.sputnikv2.testnet".to_string());
    println!("{:?}", hash);
  }

  #[test]
  fn get_snapshot() {
    // Basic set up for a unit test
    testing_env!(VMContextBuilder::new().build());
    let mut contract = Snapshot::new();
    let hash = contract.set_snapshot("jacktest.sputnikv2.testnet".to_string());
    let info = contract.get_snapshot(hash);
    println!("{:?}", info);
  }
}
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{bs58, env, near_bindgen, PanicOnDefault, AccountId};

pub mod utils;
pub mod view;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Snapshot {
  public_key: String,
  details: UnorderedMap<String, Info>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Info {
  block_height: u64,
  contract_address: AccountId,
  hash: String,
}

#[near_bindgen]
impl Snapshot {
  #[init]
  pub fn new(public_key: String) -> Self {
    assert!(!env::state_exists(), "Already initialized");
    Self {
      details: UnorderedMap::new(b"d".to_vec()),
      public_key,
    }
  }

  pub fn set_snapshot(&mut self, contract_address: AccountId, timestamp: U64, sign: String) -> String {
    let timestamp = u64::from(timestamp);
    assert!(env::block_timestamp() - timestamp < 120_000_000_000, "signature expired");
    let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
    let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();
    let json = json!(env::predecessor_account_id().to_string() + &timestamp.to_string()).to_string();
    utils::verify(json.into_bytes(), sign.into(), pk.into());

    let hash = utils::get_hash(env::block_height(), contract_address.clone());
    let info = Info {
      block_height: env::block_height(),
      contract_address: contract_address.clone(),
      hash: hash.clone(),
    };
    self.details.insert(&info.hash, &info);
    hash.clone()
  }

  pub fn delete_snapshot(&mut self, hash: String, timestamp: U64, sign: String) {
    let timestamp = u64::from(timestamp);
    assert!(env::block_timestamp() - timestamp < 120_000_000_000, "signature expired");
    let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
    let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();
    let json = json!(env::predecessor_account_id().to_string() + &timestamp.to_string()).to_string();
    utils::verify(json.into_bytes(), sign.into(), pk.into());
  
    self.details.remove(&hash);
  }
}
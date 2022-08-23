use crate::*;

use core::convert::TryFrom;
use ed25519_dalek::Verifier;
use near_sdk::{env, log, Balance, Promise, StorageUsage};

pub(crate) fn get_hash(block_height: u64, contract_address: AccountId) -> String {
  let args_string = json!({
      "block_height": block_height,
      "contract_address": contract_address,
  }).to_string();
  let hash = bs58::encode(env::sha256(args_string.as_bytes())).into_string();
  hash
}

pub(crate) fn verify(message: Vec<u8>, sign: Vec<u8>, pk: Vec<u8>) {
  let pk = ed25519_dalek::PublicKey::from_bytes(&pk).unwrap();
  if sign.len() != 64 {
      panic!("Invalid signature data length.");
  }
  let mut sig_data: [u8; 64] = [0; 64];
  for i in 0..64 {
      sig_data[i] = sign.get(i).unwrap_or(&0).clone();
  }
  let sign = ed25519_dalek::Signature::try_from(sig_data).unwrap();
  match pk.verify(&message, &sign) {
      Ok(_) => log!("verify ok"),
      Err(_) => panic!("verify error"),
  }
}
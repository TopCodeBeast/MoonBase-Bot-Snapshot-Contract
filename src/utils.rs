use crate::*;

pub(crate) fn get_hash(block_height: u64, contract_address: String) -> String {
  let args_string = json!({
      "block_height": block_height,
      "contract_address": contract_address,
  }).to_string();
  let hash = bs58::encode(env::sha256(args_string.as_bytes())).into_string();
  hash
}
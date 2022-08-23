use crate::*;

#[near_bindgen]
impl Snapshot {
  pub fn get_snapshot(&self, hash: String) -> Info{
    let info = self.details.get(&hash).unwrap();
    Info {
      hash: info.hash,
      block_height: info.block_height,
      contract_address: info.contract_address
    }
  }
}
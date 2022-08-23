# Nepbot-snapshot API

## set_snapshot(&mut self, contract_address: AccountId, timestamp: U64, sign: String)
#### add a snapshot for the contract
return hash

## delete_snapshot(&mut self, hash: String, timestamp: U64, sign: String)
#### delete snapshot based on hash


## get_snapshot(&self, hash: String) -> Info
#### get snapshot based on the hash
return info

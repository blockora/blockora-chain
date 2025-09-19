
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub data: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, previous_hash: String, data: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            data,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let record = format!("{}{}{}{}", self.index, self.timestamp, self.previous_hash, self.data);
        let mut hasher = Sha256::new();
        hasher.update(record);
        let result = hasher.finalize();
        hex::encode(result)
    }
}

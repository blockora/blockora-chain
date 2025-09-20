use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub data: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: String, nonce: u64) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            data,
            nonce,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let record = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn is_valid(&self) -> bool {
        self.hash == self.calculate_hash()
    }
}

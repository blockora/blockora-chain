use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, &data, &prev_hash));
        let result = hasher.finalize();
        let hash = format!("{:x}", result);

        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }

    pub fn genesis() -> Self {
        Block::new(0, "Genesis Block".to_string(), "0".to_string())
    }
}

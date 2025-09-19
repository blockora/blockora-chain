use crate::core::block::Block;

/// Simple in-memory blockchain
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    /// Create a new blockchain with genesis block
    pub fn new() -> Self {
        Self { chain: vec![Block::genesis()] }
    }

    /// Return last block's hash
    pub fn last_hash(&self) -> String {
        self.chain.last().unwrap().hash.clone()
    }

    /// Add a new block with provided data (data can be JSON of transactions)
    pub fn add_block(&mut self, data: String) -> &Block {
        let index = self.chain.len() as u64;
        let prev_hash = self.last_hash();
        let block = Block::new(index, data, prev_hash);
        self.chain.push(block);
        self.chain.last().unwrap()
    }

    /// Basic chain validation: check prev_hash linkage and recalculated hash
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let prev = &self.chain[i - 1];

            if current.prev_hash != prev.hash {
                return false;
            }

            // Recalculate hash and compare
            let recalculated = {
                use sha2::{Sha256, Digest};
                let record = format!("{}{}{}{}", current.index, current.timestamp, current.data, current.prev_hash);
                let mut hasher = Sha256::new();
                hasher.update(record);
                let result = hasher.finalize();
                format!("{:x}", result)
            };

            if recalculated != current.hash {
                return false;
            }
        }
        true
    }
}

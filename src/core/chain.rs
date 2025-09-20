use crate::core::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain { chain: vec![] };
        blockchain.add_block("Genesis Block".to_string());
        blockchain
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = if self.chain.is_empty() {
            String::from("0")
        } else {
            self.latest_block().hash.clone()
        };
        let new_block = Block::new(self.chain.len() as u64, previous_hash, data, 0);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if !current.is_valid() {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

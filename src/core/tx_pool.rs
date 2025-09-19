use crate::wallet::wallet::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TxPool {
    pub pool: Vec<Transaction>,
}

impl TxPool {
    pub fn new() -> Self {
        Self { pool: Vec::new() }
    }

    pub fn add_tx(&mut self, tx: Transaction) {
        self.pool.push(tx);
    }

    /// Pop all transactions and return them; pool becomes empty
    pub fn take_all(&mut self) -> Vec<Transaction> {
        let txs = self.pool.clone();
        self.pool.clear();
        txs
    }

    pub fn len(&self) -> usize {
        self.pool.len()
    }
}

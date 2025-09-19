use rand::RngCore;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use hex;

/// Simple prototype wallet (educational). Uses a random 32-byte private key stored in memory.
/// Address = first 20 bytes of SHA256(public/private), signature = SHA256(private_key || message)
#[derive(Debug, Clone)]
pub struct Wallet {
    pub private_key: Vec<u8>,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let mut priv_key = vec![0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut priv_key);
        let mut hasher = Sha256::new();
        hasher.update(&priv_key);
        let digest = hasher.finalize();
        let address = hex::encode(&digest[..20]); // short 20-byte hex address
        Wallet { private_key: priv_key, address }
    }

    /// Sign message (prototype): SHA256(private_key || message)
    pub fn sign_message(&self, message: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.private_key);
        hasher.update(message);
        let sig = hasher.finalize();
        sig.to_vec()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn new(wallet: &Wallet, to: String, amount: u64) -> Self {
        let mut tx_data = vec![];
        tx_data.extend(wallet.address.as_bytes());
        tx_data.extend(to.as_bytes());
        tx_data.extend(amount.to_le_bytes());

        let signature = wallet.sign_message(&tx_data);
        Transaction {
            from: wallet.address.clone(),
            to,
            amount,
            signature,
        }
    }
}

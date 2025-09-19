use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, PublicKey, Signer};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Wallet {
    pub keypair: Keypair,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let address = Self::public_key_to_address(&keypair.public);
        Wallet { keypair, address }
    }

    fn public_key_to_address(pubkey: &PublicKey) -> String {
        let mut hasher = Sha256::new();
        hasher.update(pubkey.as_bytes());
        let result = hasher.finalize();
        hex::encode(&result[..20])
    }

    pub fn sign_message(&self, message: &[u8]) -> Vec<u8> {
        self.keypair.sign(message).to_bytes().to_vec()
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

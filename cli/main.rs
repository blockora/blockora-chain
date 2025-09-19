use blockora_chain::core::chain::Blockchain;
use blockora_chain::wallet::wallet::{Wallet, Transaction};

pub fn main() {
    println!("[OK] Blockora CLI Started!");

    let mut blockchain = Blockchain::new();
    println!("[OK] Blockchain created with Genesis Block.");

    let wallet_a = Wallet::new();
    let wallet_b = Wallet::new();
    println!("[INFO] Wallet A Address: {}", wallet_a.address);
    println!("[INFO] Wallet B Address: {}", wallet_b.address);

    let tx1 = Transaction::new(&wallet_a, wallet_b.address.clone(), 100);
    println!("[TX] Created Transaction from Wallet A to B");
    
    let tx_data = serde_json::to_string(&tx1).unwrap();
    let new_block = blockchain.add_block(tx_data);
    println!("[MINE] Mined new Block {} with hash {}", new_block.index, new_block.hash);
    
    println!("[INFO] Blockchain length: {}", blockchain.chain.len());

    if blockchain.is_valid() {
        println!("[OK] Blockchain is valid.");
    } else {
        println!("[ERR] Blockchain is NOT valid.");
    }
}

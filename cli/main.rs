use blockora_chain::wallet::wallet::{Wallet, Transaction};
use blockora_chain::network::p2p;

fn main() {
    println!("🚀 Blockora CLI Started!");

    // Create wallet
    let wallet = Wallet::new();
    println!("💳 Wallet Created!");
    println!("🔑 Address: {}", wallet.address);

    // Create test transaction
    let tx = Transaction::new(&wallet, "receiver123".to_string(), 50);
    println!("✅ Transaction Created: {:?}", tx);

    // Start P2P Network
    println!("🌐 Starting P2P Node...");
    p2p::start_server("127.0.0.1:6000");
}

use std::thread;
use std::time::Duration;

use blockora_chain::core::chain::Blockchain;
use blockora_chain::core::tx_pool::TxPool;
use blockora_chain::wallet::wallet::{Wallet, Transaction};
use blockora_chain::network::p2p;

fn main() {
    println!("🚀 Blockora CLI Started!");

    // Start P2P server on a separate thread (non-blocking for this demo)
    let _server_thread = thread::spawn(|| {
        p2p::start_server("127.0.0.1:6000");
    });

    // Create a simple in-memory blockchain and tx pool
    let mut blockchain = Blockchain::new();
    let mut pool = TxPool::new();

    // Create two wallets
    let wallet_a = Wallet::new();
    let wallet_b = Wallet::new();

    println!("💳 Wallet A Address: {}", wallet_a.address);
    println!("💳 Wallet B Address: {}", wallet_b.address);

    // Create a transaction from A -> B and add to pool
    let tx = Transaction::new(&wallet_a, wallet_b.address.clone(), 100);
    println!("✅ Created Transaction: from {} to {} amount {}", tx.from, tx.to, tx.amount);
    pool.add_tx(tx);

    // simulate waiting for more txs
    println!("⏳ Waiting to collect transactions...");
    thread::sleep(Duration::from_secs(2));

    // Mine: take all txs from pool, serialize to JSON, and create a block with that data
    if pool.len() > 0 {
        let txs = pool.take_all();
        let txs_json = serde_json::to_string(&txs).unwrap();
        let new_block = blockchain.add_block(txs_json);
        println!("⛏️ Mined new block: index {} hash {}", new_block.index, new_block.hash);
    } else {
        println!("No transactions to mine.");
    }

    // Print full chain summary
    println!("📚 Blockchain length: {}", blockchain.chain.len());
    for b in blockchain.chain.iter() {
        println!(" - idx={} hash={} prev={}", b.index, b.hash, b.prev_hash);
    }

    // Keep CLI alive so P2P server thread can run for demo (Ctrl+C to exit)
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

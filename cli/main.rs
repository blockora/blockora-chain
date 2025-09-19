use blockora_chain::core::block::Block;
use blockora_chain::network::p2p;

fn main() {
    println!("🚀 Blockora CLI Started!");

    // Start Genesis Block
    let genesis = Block::genesis();
    println!("✅ Genesis Block Created: {:?}", genesis);

    // Start P2P Network
    println!("🌐 Starting P2P Node...");
    p2p::start_server("127.0.0.1:6000");
}

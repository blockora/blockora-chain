use std::time::{SystemTime, UNIX_EPOCH};
mod core;

fn main() {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let genesis_block = core::block::Block::new(0, timestamp, String::from("0"), String::from("Genesis Block"));
    println!("Genesis Block Created: {:?}", genesis_block);
}

use blockora_chain::core::chain::Blockchain;
use blockora_chain::core::block::Block;

#[test]
fn test_genesis_and_add_block() {
    let mut bc = Blockchain::new();
    assert_eq!(bc.chain.len(), 1);
    assert_eq!(bc.chain[0].index, 0);
    let _ = bc.add_block("test-data".to_string());
    assert_eq!(bc.chain.len(), 2);
    assert!(bc.is_valid());
}

#[test]
fn test_basic() {
    assert_eq!(2 + 2, 4);
}

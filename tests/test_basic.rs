use blockora::core::chain::Blockchain;

#[test]
fn test_blockchain_creation() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Test Block".to_string());

    assert!(blockchain.is_valid());
    assert_eq!(blockchain.chain.len(), 2); // Genesis + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ now, Block, Blockchain };

    fn create_test_blockchain() -> Blockchain {
        let difficulty = 0x000fffffffffffffffffffffffffffff;
        let mut block = Block::new(0, now(), vec![0; 32], 0, "Genesis block!".to_owned(), difficulty);
        block.mine();
        let mut last_hash = block.hash.clone();
        let mut blockchain = Blockchain {
            blocks: vec![block],
        };
        for i in 1..=10 {
            let mut block = Block::new(i, now(), last_hash, 0, "Another block".to_owned(), difficulty);
    
            block.mine();
    
            last_hash = block.hash.clone();
    
            blockchain.blocks.push(block);
        }
        blockchain
    }
    #[test]
    fn index_verification() {
        let mut blockchain = create_test_blockchain();
        blockchain.blocks[3].index = 4;
        assert!(!&blockchain.verify());
    }
    #[test]
    fn difficulty_verification() {
        let mut blockchain = create_test_blockchain();
        blockchain.blocks[3].payload = "No".to_owned();
        assert!(!&blockchain.verify());
    }
    #[test]
    fn time_verification() {
        let mut blockchain = create_test_blockchain();
        blockchain.blocks[3].timestamp = 0;
        assert!(!&blockchain.verify());
    }
    #[test]
    fn hash_verification() {
        let mut blockchain = create_test_blockchain();
        blockchain.blocks[3].hash[8] += 1;
        assert!(!&blockchain.verify());
    }
    #[test]
    fn genesis_hash_verification() {
        let mut blockchain = create_test_blockchain();
        blockchain.blocks[0].hash[1] += 1;
        assert!(!&blockchain.verify());
    }
}
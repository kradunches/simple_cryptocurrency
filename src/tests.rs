#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ now, transaction, Block, Blockchain, Transaction };
    #[test]
    fn invalid_input() {
        let difficulty = 0x000fffffffffffffffffffffffffffff;
        //First transaction (genesis)
        let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
            Transaction{
                inputs: vec![ ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 50,
                    },transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 7,
                    }
                ]
        }], difficulty);
    
        genesis_block.mine();
        println!("Mined genesids block {:?}", &genesis_block);
    
        let mut last_hash = genesis_block.hash.clone();
    
        let mut blockchain = Blockchain::new();
        
        blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");
    
        // Second transaction
        let mut block = Block::new(1, now(), last_hash, vec![
            Transaction {
                inputs: vec![ ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Chris".to_owned(),
                        value: 536,
                    },
                ],
            },
            Transaction {
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[0].clone(),
                    transaction::Output {
                        to_addr: "Bodfjkldfj".to_owned(),
                        value: 3432424,
                    },
                ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 34,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
        ], difficulty);
    
        block.mine();
    
        println!("Mined block {:?}", &block);
    
        last_hash = block.hash.clone();
    
        assert!(blockchain.update_with_block(block).is_err());
    }
    #[test]
    fn spending_more_than_you_have() {
        let difficulty = 0x000fffffffffffffffffffffffffffff;
        //First transaction (genesis)
        let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
            Transaction{
                inputs: vec![ ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 50,
                    },transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 7,
                    }
                ]
        }], difficulty);
    
        genesis_block.mine();
        println!("Mined genesids block {:?}", &genesis_block);
    
        let mut last_hash = genesis_block.hash.clone();
    
        let mut blockchain = Blockchain::new();
        
        blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");
    
        // Second transaction
        let mut block = Block::new(1, now(), last_hash, vec![
            Transaction {
                inputs: vec![ ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Chris".to_owned(),
                        value: 536,
                    },
                ],
            },
            Transaction {
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[0].clone(),
                ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 34443,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12234,
                    },
                ],
            },
        ], difficulty);
    
        block.mine();
    
        println!("Mined block {:?}", &block);
    
        last_hash = block.hash.clone();
    
        assert!(blockchain.update_with_block(block).is_err());
    }
}
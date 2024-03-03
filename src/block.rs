use std::fmt::Debug;

use super::BlockHash;

pub struct Block {
    pub index: u32,
    pub nonce: u64,
    pub timestamp: u128,
    pub payload: String,
    pub block_hash: BlockHash,
    pub prev_block_hash: BlockHash,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block[{}]: \n Block Hash: {} \n timestamp: {} \n payload: {} ",
            &self.index,
            &hex::encode(&self.block_hash),
            &self.timestamp,
            &self.payload,
        )
    }
}

// this has a fn to create blocks
impl Block {
    pub fn create_new_block(
        index: u32,
        nonce: u64,
        timestamp: u128,
        payload: String,
        prev_block_hash: BlockHash,
    ) -> Self {
        Self {
            index,
            nonce,
            timestamp,
            payload,
            prev_block_hash,
            // we make here hash with the help of the above data in future
            block_hash: vec![0; 32],
        }
    }
}

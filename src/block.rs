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

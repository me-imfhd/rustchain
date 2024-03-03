use std::fmt::Debug;

use crate::{u128_bytes, u32_bytes, u64_bytes, Hashable};

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
            "\nBlock[{}]: \nBlock Hash: {} \ntimestamp: {} \npayload: {} \n",
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

impl Hashable for Block {
    fn block_to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // convert index which is u32 to an array of u8 values and extending that to bytes variable
        bytes.extend(&u32_bytes(&self.index));
        // convert nonce (u64) to array of u8
        bytes.extend(&u64_bytes(&self.nonce));
        // convert timestamp (u128) to array of u8
        bytes.extend(&u128_bytes(&self.timestamp));
        // convert payload to byte
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&self.prev_block_hash);

        bytes
    }
}

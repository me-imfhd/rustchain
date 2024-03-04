use std::fmt::Debug;

use crate::{difficulty_bytes_as_u128, u128_bytes, u32_bytes, u64_bytes, Hashable};

use super::BlockHash;

pub struct Block {
    pub index: u32,
    pub nonce: u64,
    pub timestamp: u128,
    pub payload: String,
    pub block_hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nBlock[{}]: \nBlock Hash: {} \ntimestamp: {} \npayload: {} \nnonce: {} \nPrevious Block Hash: {} \n",
            &self.index,
            &hex::encode(&self.block_hash),
            &self.timestamp,
            &self.payload,
            &self.nonce,
            &hex::encode(&self.prev_block_hash),
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
        difficulty: u128,
    ) -> Self {
        Self {
            index,
            nonce,
            timestamp,
            payload,
            prev_block_hash,
            block_hash: vec![0; 32],
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for nonce_incrementor in 0..(u64::max_value()) {
            self.nonce = nonce_incrementor;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.block_hash = hash;
                return;
            }
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

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(hash)
}

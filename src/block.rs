use super::BlockHash;

pub struct Block {
    pub index : u32,
    pub nonce : u64,
    pub timestampt : u128,
    pub payload : String,
    pub block_hash : BlockHash,
    pub prev_block_hash : BlockHash
}

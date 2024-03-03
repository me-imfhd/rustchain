pub trait Hashable {
    fn block_to_bytes(&self) -> Vec<u8>; // writing fn declaration will implement this in block struct

    fn hash(&self) -> Vec<u8> { // implementing hashing fn for the block we mentioned while creating block hash
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.block_to_bytes())
    }
}

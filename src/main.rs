use blockchainlib::{Block, Hashable};
fn main() {
    let mut block = Block::create_new_block(0, 0, 0, String::from("Payload"), vec![0; 32]);
    println!("{:?}", &block);
    let hash = block.hash();
    println!("{:?}", &hash);

    block.block_hash = hash;

    println!("{:?}", &block)
}

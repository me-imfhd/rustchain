use blockchainlib::{get_current_time, Block, Hashable};
fn main() {
    let mut block = Block::create_new_block(
        0,
        0,
        get_current_time(),
        String::from("Payload"),
        vec![0; 32],
        0x0000ffffffffffffffffffffffffffff,
    );
    println!("{:?}", &block);

    block.block_hash = block.hash();

    block.mine();

    println!("{:?}", &block)
}

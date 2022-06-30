/*
    Appellation: mod
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */
pub use blocks::*;
pub use chains::*;

mod blocks;
mod chains;

pub type BlockId = u64;
pub type BlockData = String;
pub type BlockHash = String;
pub type BlockNonce = u64;
pub type BTimestamp = i64;

mod utils {
    use super::*;

    pub fn block_miner() -> (BlockNonce, BlockHash) {
        todo!()
    }
}
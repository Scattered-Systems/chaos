use acme::chain::blockchain::Block;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
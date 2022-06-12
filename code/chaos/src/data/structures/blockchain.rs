use acme::chain::blockchain::{Block, types::Transaction};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chain {
    pub blocks: Vec<Block>,
}
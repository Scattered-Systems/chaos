use acme::{
    chains::blockchain::Block,
    primitives::containers::Transaction
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        let nonce = 90890;
        let previous: String = String::from("");
        let transactions = vec![Transaction::new()];
        let mut blocks = vec![Block::new(nonce, previous, transactions)];
        Self {
            blocks
        }
    }
}
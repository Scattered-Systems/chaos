use acme::{
    primitives::{Clock, ObjectId, Transaction},
    utils::date::timestamp,
};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Block {
    pub id: ObjectId,
    pub hash: String,
    pub nonce: usize,
    pub previous: String,
    pub timestamp: Clock,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(nonce: usize, previous: String, transactions: Vec<Transaction>) -> Self {
        Self {
            id: ObjectId::new(),
            hash: String::from(""),
            nonce,
            previous,
            timestamp: timestamp(),
            transactions,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
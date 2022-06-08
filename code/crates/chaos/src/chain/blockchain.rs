use std::collections::HashMap;

use acme::primitives::{Clock, ObjectId};
use chrono;
use serde::{Deserialize, Serialize};

pub type Transaction = HashMap<String, String>;
pub type Bundle = [Transaction; 16];


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Block {
    pub id: ObjectId,
    pub hash: String,
    pub nonce: String,
    pub previous: String,
    pub timestamp: Clock,
    pub transactions: Bundle,
}

impl Block {
    pub fn new(nonce: String, previous: String, transactions: Bundle) -> Self {
        Self {
            id: ObjectId::new(),
            hash: String::from(""),
            nonce,
            previous,
            timestamp: chrono::Local::now().into(),
            transactions,
        }
    }
}
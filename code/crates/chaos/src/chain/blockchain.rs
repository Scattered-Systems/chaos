use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use acme::primitives::{Clock, ObjectId};

pub struct Transaction {
    pub timestamp: Clock,
}

pub struct Block {
    pub id: ObjectId,
    pub hash: String,
    pub key: String,
    pub timestamp: Clock,
}
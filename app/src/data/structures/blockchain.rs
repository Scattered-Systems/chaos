use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blockchain<T> {
    pub blocks: Vec<T>,
}
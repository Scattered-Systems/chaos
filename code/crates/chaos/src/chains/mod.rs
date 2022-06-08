pub mod blockchain;
pub mod networking;

pub mod primitives {
    use std::collections::HashMap;

    pub type Transaction = HashMap<String, String>;
    pub type Bundle = [Transaction; 16];
}
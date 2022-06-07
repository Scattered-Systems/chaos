use std::{collections::HashMap, net::SocketAddr};

#[derive(Clone, Debug)]
pub struct Node {
    pub addresses: HashMap<String, SocketAddr>,
}

impl Node {
    pub fn new() -> Self {
        let host = [0, 0, 0, 0];
        let ports = [8088, 8089, 8090];
        let labels = ["api", "client", "chain"];
        let mut addresses = HashMap::new();
        for i in 0..3 {
            addresses.insert(String::from(labels[i]), SocketAddr::from((host, ports[i])));
        }
        Self {
            addresses
        }
    }
}
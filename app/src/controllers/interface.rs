use acme::Peer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Interface {
    pub configuration: crate::Settings,
}

impl Interface {
    pub fn new(configuration: crate::Settings) -> Self {
        Self {
            configuration
        }
    }

    pub fn constructor() -> Peer {
        let peer = Peer::new();
        return peer
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interface(settings={})", self.configuration)
    }
}
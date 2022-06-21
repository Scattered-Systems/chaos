use acme::Peer;
use serde::{Deserialize, Serialize};

pub struct Interface {
    pub peer: Peer,
}

impl Interface {
    pub fn new(settings: crate::Settings) -> Self {
        Self {
            peer: Peer::new()
        }
    }
}

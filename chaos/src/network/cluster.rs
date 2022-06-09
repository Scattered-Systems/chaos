use crate::network::conduit::Conduit;

// TODO - Create a simple interface that allows for swarm's to be quickly initialized
#[derive(Clone)]
pub struct Cluster {
    pub conduit: Conduit,
}

impl Cluster {
    pub fn new(conduit: Conduit) -> Self {
        Self {
            conduit: conduit.clone(),
        }
    }
}
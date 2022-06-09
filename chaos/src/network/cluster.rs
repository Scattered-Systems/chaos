use libp2p::{
    mdns::Mdns,
    NetworkBehaviour,
    swarm::{Swarm, SwarmBuilder, SwarmEvent},
};

trait Cusp {
    type Behaviour;
    fn swarm(&mut self) -> Swarm<Behaviour>;
}

pub struct Cluster {
    pub conduit: crate::network::conduit::Conduit,
    pub swarm: Swarm<TBehaviour>,
}

impl Cluster {
    pub fn new(behaviour: Behaviour) -> SwarmBuilder {
        let mut swarm = {
            let mdns = Mdns::new(Default::default()).await?;
            SwarmBuilder::new(transport, behaviour, peer.id.clone())
                .executor(Box::new(|fut| {
                    tokio::spawn(fut);
                }))
                .build()
        };
    }
}
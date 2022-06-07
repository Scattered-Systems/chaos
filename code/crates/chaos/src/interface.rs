use libp2p::swarm;
use tokio::{io::{self, AsyncBufReadExt}, select};

use crate::chain::peer::{Peer, Provider};

pub struct Interface {
    pub peer: Peer,
}

impl Interface {
    pub fn new() -> Self {
        let peer = Peer::new();
        Self {
            peer: peer.clone()
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let provider = Provider::new(&self.peer.clone());
        let transport = provider.transport.clone();

        loop {
            select! {
                // line = stdin.next_line() => {
                //     let line = line?.expect("stdin closed");
                //     swarm.behaviour_mut().floodsub.publish(floodsub_topic.clone(), line.as_bytes());
                // }
                event = swarm.select_next_some() => {
                    if let SwarmEvent::NewListenAddr { address, .. } = event {
                        println!("Listening on {:?}", address);
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Peer(id={})", self.peer.id.clone())
    }
}
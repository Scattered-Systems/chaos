use futures::StreamExt;
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::Boxed},
    floodsub::{self, Floodsub, FloodsubEvent},
    mdns::{Mdns, MdnsEvent},
    mplex,
    Multiaddr,
    NetworkBehaviour,
    PeerId,
    Swarm,
    swarm::{NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
};
use tokio::{
    io::{self, AsyncBufReadExt},
    select,
};

use crate::network::{behaviour::CustomBehaviour, peer::Peer};

#[derive(Clone, Debug)]
pub struct Interface {
    pub peer: Peer,
}

impl Interface {
    pub fn new() -> Self {
        let peer = Peer::new();
        Self {
            peer: peer.clone(),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        pretty_env_logger::init();

        let floodsub_topic: floodsub::Topic = floodsub::Topic::new("chat");

        let peer = Peer::new();
        let transport = Peer::transport(&peer);
        println!("{}", peer.clone());

        let mut swarm = {
            let mdns = Mdns::new(Default::default()).await?;
            let mut behaviour = CustomBehaviour {
                floodsub: Floodsub::new(peer.id.clone()),
                mdns,
            };

            behaviour.floodsub.subscribe(floodsub_topic.clone());

            SwarmBuilder::new(transport, behaviour, peer.id.clone())
                .executor(Box::new(|fut| {
                    tokio::spawn(fut);
                }))
                .build()
        };

        // If specified, dial the node at the provided address
        if let Some(to_dial) = std::env::args().nth(1) {
            let addr: Multiaddr = to_dial.parse()?;
            swarm.dial(addr)?;
            println!("Dialed {:?}", to_dial);
        }

        // Read full lines from stdin
        let mut stdin = io::BufReader::new(io::stdin()).lines();

        // Listen on all interfaces and whatever port the OS assigns
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        // Run
        loop {
            select! {
                line = stdin.next_line() => {
                    let line = line?.expect("stdin closed");
                    swarm.behaviour_mut().floodsub.publish(floodsub_topic.clone(), line.as_bytes());
                }
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
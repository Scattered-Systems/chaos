use acme::primitives::StandardError;
use futures::StreamExt;
use libp2p::{
    floodsub::{Floodsub, Topic},
    mdns::Mdns,
    Multiaddr,
    swarm::{SwarmBuilder, SwarmEvent},
};
use tokio::io::AsyncBufReadExt;

use crate::network::{behaviour::FloodsubMdnsBehaviour, conduit::Conduit, peer::Peer};

// TODO: Move primitive type into the acme crate
pub type NetworkAddress = Multiaddr;

#[derive(Clone)]
pub struct Node {
    pub conduit: Conduit,
    pub peer: Peer,
}

impl Node {
    pub fn new() -> Self {
        let peer = Peer::new();
        let conduit = Conduit::new(&peer);
        Self {
            conduit: conduit.clone(),
            peer: peer.clone(),
        }
    }
    pub async fn run(&self) -> Result<(), StandardError> {
        pretty_env_logger::init();

        let floodsub_topic: Topic = Topic::new("chat");

        let peer = Peer::new();
        let transport = Peer::transport(&peer);
        println!("{}", peer.clone());

        let mut swarm = {
            let mdns = Mdns::new(Default::default()).await?;
            let mut behaviour = FloodsubMdnsBehaviour {
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
            let addr: NetworkAddress = to_dial.parse()?;
            swarm.dial(addr)?;
            println!("Dialed {:?}", to_dial);
        }

        // Read full lines from stdin
        let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

        // Listen on all interfaces and whatever port the OS assigns
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        // Run
        loop {
            tokio::select! {
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
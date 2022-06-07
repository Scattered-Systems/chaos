use std::error::Error;

use futures::StreamExt;
use libp2p::{
    core::upgrade,
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsEvent},
    mplex,
    Multiaddr,
    NetworkBehaviour,
    // `TokioTcpConfig` is available through the `tcp-tokio` feature.
    noise,
    PeerId,
    swarm::{dial_opts::DialOpts, NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
    tcp::TokioTcpConfig,
    Transport,
};
use tokio::io::{self, AsyncBufReadExt};

mod chain;
mod data;
mod interface;
mod settings;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    pretty_env_logger::init();

    // Create a local peer
    let local = crate::chain::peer::Peer::new();
    println!("{}", local.clone());

    // Create a keypair for authenticated encryption of the transport.
    let noise_keys = crate::chain::peer::Peer::authorize(local.key.clone());

    let provider = crate::chain::peer::Provider::new(local.clone());
    let transport = provider.transport.clone();


    let floodsub_topic = floodsub::Topic::new("chat");


    #[derive(NetworkBehaviour)]
    #[behaviour(event_process = true)]
    struct MyBehaviour {
        floodsub: Floodsub,
        mdns: Mdns,
    }

    impl NetworkBehaviourEventProcess<FloodsubEvent> for MyBehaviour {
        // Called when `floodsub` produces an event.
        fn inject_event(&mut self, message: FloodsubEvent) {
            if let FloodsubEvent::Message(message) = message {
                println!(
                    "Received: '{:?}' from {:?}",
                    String::from_utf8_lossy(&message.data),
                    message.source
                );
            }
        }
    }

    impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
        // Called when `mdns` produces an event.
        fn inject_event(&mut self, event: MdnsEvent) {
            match event {
                MdnsEvent::Discovered(list) => {
                    for (peer, _) in list {
                        self.floodsub.add_node_to_partial_view(peer);
                    }
                }
                MdnsEvent::Expired(list) => {
                    for (peer, _) in list {
                        if !self.mdns.has_node(&peer) {
                            self.floodsub.remove_node_from_partial_view(&peer);
                        }
                    }
                }
            }
        }
    }

    // Create a Swarm to manage peers and events.
    let mut swarm = {
        let mdns = Mdns::new(Default::default()).await?;
        let mut behaviour = MyBehaviour {
            floodsub: Floodsub::new(local.id.clone()),
            mdns,
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());

        SwarmBuilder::new(transport, behaviour, local.id.clone())
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build()
    };

    // Reach out to another node if specified
    if let Some(to_dial) = std::env::args().nth(1) {
        let addr: Multiaddr = to_dial.parse()?;
        swarm.dial(addr)?;
        println!("Dialed {:?}", to_dial);
    }

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Kick it off
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
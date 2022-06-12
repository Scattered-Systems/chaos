use acme::{
    chain::networking::peer::Peer,
    primitives::{
        errors::DynamicError,
        identifiers::NetworkAddress,
        networking::BoxedTransport,
    },
};
use futures::StreamExt;
use libp2p::{
    kad::{
        Kademlia,
        Quorum,
        Record,
        record::{
            Key,
            store::MemoryStore,
        },
    },
    mdns::{Mdns, MdnsConfig},
    swarm::{SwarmBuilder, SwarmEvent},
    Swarm,
};
use tokio::{io::{self, AsyncBufReadExt}, task};

use crate::network::behaviours::storage::StorageBehaviour;
use crate::settings::Settings;

// TODO - Refine the structure into a trait and/or proc_macro

#[derive(Clone)]
pub struct Node {
    pub peer: Peer,
    pub settings: Settings,
}

impl Node {
    pub fn new(settings: Settings) -> Self {
        let peer = Peer::new();
        Self {
            peer: peer.clone(),
            settings: settings.clone(),
        }
    }

    pub async fn run(self) -> Result<(), DynamicError> {
        pretty_env_logger::init();

        let peer = self.peer.clone();
        let transport = Peer::build_transport(&peer);
        println!("{}", peer.clone());

        let store = MemoryStore::new(peer.id.clone());
        let kademlia = Kademlia::new(peer.id.clone(), store);
        let mdns = Mdns::new(MdnsConfig::default()).await?;
        let mut behaviour = StorageBehaviour { kademlia, mdns };

        let mut swarm = {
            SwarmBuilder::new(transport, behaviour, peer.id.clone())
                .executor(Box::new(|fut| { tokio::spawn(fut); }))
                .build()
        };

        // If specified, dial the node at the provided address
        if let Some(to_dial) = std::env::args().nth(1) {
            let addr: NetworkAddress = to_dial.parse()?;
            swarm.dial(addr)?;
            println!("Dialed {:?}", to_dial);
        }

        // Read full lines from stdin
        let mut stdin = io::BufReader::new(io::stdin()).lines();

        // Listen on all interfaces and whatever port the OS assigns
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        // Run
        loop {
            tokio::select! {
                line = stdin.next_line() => handle_input_line(&mut swarm.behaviour_mut().kademlia, line?.expect("Stdin not to close")),
                event = swarm.select_next_some() => match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Listening in {:?}", address);
                    },
                    _ => {}
                }
            }
        }
    }
}

fn handle_input_line(kademlia: &mut Kademlia<MemoryStore>, line: String) {
    let mut args = line.split(' ');
    match args.next() {
        Some("GET") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            kademlia.get_record(key, Quorum::One);
        }
        Some("GET_PROVIDERS") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            kademlia.get_providers(key);
        }
        Some("PUT") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            let value = {
                match args.next() {
                    Some(value) => value.as_bytes().to_vec(),
                    None => {
                        eprintln!("Expected value");
                        return;
                    }
                }
            };
            let record = Record {
                key,
                value,
                publisher: None,
                expires: None,
            };
            kademlia
                .put_record(record, Quorum::One)
                .expect("Failed to store record locally.");
        }
        Some("PUT_PROVIDER") => {
            let key = {
                match args.next() {
                    Some(key) => Key::new(&key),
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };

            kademlia
                .start_providing(key)
                .expect("Failed to start providing key");
        }
        _ => {
            eprintln!("expected GET, GET_PROVIDERS, PUT or PUT_PROVIDER");
        }
    }
}
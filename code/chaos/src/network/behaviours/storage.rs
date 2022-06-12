use acme::{chain::networking::peer::Peer, primitives::networking::BoxedTransport};
use libp2p::{
    kad::{
        AddProviderOk,
        Kademlia,
        KademliaEvent,
        PeerRecord,
        PutRecordOk,
        QueryResult,
        Record,
        record::store::MemoryStore,
    },
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    NetworkBehaviour,
    swarm::{NetworkBehaviourEventProcess, Swarm},
};
use tokio::{io, task};

pub type Kad = Kademlia<MemoryStore>;

pub enum Results {
    Query(QueryResult)
}

pub enum NetworkProcesses {
    Kad(KademliaEvent),
    MDNS(MdnsEvent),
}

pub struct Interface<T> {
    pub behaviour: StorageBehaviour,
    pub peer: Peer,
    pub swarm: Swarm<T>,
    pub transport: BoxedTransport,
}

impl Interface<T> {
    pub fn new(peer: Peer, transport: BoxedTransport) -> Self {
        let store = MemoryStore::new(peer.id.clone());
        let kademlia = Kademlia::new(peer.id.clone(), store);
        let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
        let behaviour = MyBehaviour { kademlia, mdns };
        let mut swarm = {
            Swarm::new(transport.clone(), behaviour, local_peer_id)
        };
        Self {
            behaviour,
            peer: peer.clone(),
            swarm,
            transport: transport.clone(),
        }
    }
}

// TODO - Finish implementing the network behaviour for storing Key-Value pairs
#[NetworkBehaviour]
#[behaviour(event_process = true)]
pub struct StorageBehaviour {
    pub kademlia: Kad,
    pub mdns: Mdns,
}

// Implement a custom process for Kademlia Event's
impl NetworkBehaviourEventProcess<KademliaEvent> for StorageBehaviour {
    fn inject_event(&mut self, message: KademliaEvent) {
        match message {
            KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
                QueryResult::GetProviders(Ok(ok)) => {
                    for peer in ok.providers {
                        println!(
                            "Peer {:?} provides key {:?}",
                            peer,
                            std::str::from_utf8(ok.key.as_ref()).unwrap()
                        );
                    }
                }
                QueryResult::GetProviders(Err(err)) => {
                    eprintln!("Failed to get providers: {:?}", err);
                }
                QueryResult::GetRecord(Ok(ok)) => {
                    for PeerRecord {
                        record: Record { key, value, .. },
                        ..
                    } in ok.records
                    {
                        println!(
                            "Got record {:?} {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap(),
                            std::str::from_utf8(&value).unwrap(),
                        );
                    }
                }
                QueryResult::GetRecord(Err(err)) => {
                    eprintln!("Failed to get record: {:?}", err);
                }
                QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                    println!(
                        "Successfully put record {:?}",
                        std::str::from_utf8(key.as_ref()).unwrap()
                    );
                }
                QueryResult::PutRecord(Err(err)) => {
                    eprintln!("Failed to put record: {:?}", err);
                }
                QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
                    println!(
                        "Successfully put provider record {:?}",
                        std::str::from_utf8(key.as_ref()).unwrap()
                    );
                }
                QueryResult::StartProviding(Err(err)) => {
                    eprintln!("Failed to put provider record: {:?}", err);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for StorageBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                self.kademlia.add_address(&peer_id, multiaddr);
            }
        }
    }
}
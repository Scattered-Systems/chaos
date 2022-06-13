use disaronno::{networking::peers::Peer, types::BoxedTransport};
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
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
    swarm::NetworkBehaviourEventProcess,
};

pub type Kad = Kademlia<MemoryStore>;

pub enum Results {
    Query(QueryResult)
}

pub enum NetworkProcesses {
    Kad(KademliaEvent),
    MDNS(MdnsEvent),
}

// TODO - Finish implementing the network behaviour for storing Key-Value pairs
#[derive(NetworkBehaviour)]
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
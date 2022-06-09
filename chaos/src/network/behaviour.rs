use libp2p::{
    floodsub::{Floodsub, FloodsubEvent},
    gossipsub::{Gossipsub, GossipsubEvent},
    identify::{Identify, IdentifyEvent},
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
    ping,
    swarm::NetworkBehaviourEventProcess,
};

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct PrivateIpfsNetwork {
    pub gossipsub: Gossipsub,
    pub identify: Identify,
    pub ping: ping::Behaviour,
}

impl NetworkBehaviourEventProcess<IdentifyEvent> for PrivateIpfsNetwork {
    // Called when `identify` produces an event.
    fn inject_event(&mut self, event: IdentifyEvent) {
        println!("identify: {:?}", event);
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for PrivateIpfsNetwork {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            } => println!(
                "Got message: {} with id: {} from peer: {:?}",
                String::from_utf8_lossy(&message.data),
                id,
                peer_id
            ),
            _ => {}
        }
    }
}

impl NetworkBehaviourEventProcess<ping::Event> for PrivateIpfsNetwork {
    // Called when `ping` produces an event.
    fn inject_event(&mut self, event: ping::Event) {
        match event {
            ping::Event {
                peer,
                result: Result::Ok(ping::Success::Ping { rtt }),
            } => {
                println!(
                    "ping: rtt to {} is {} ms",
                    peer.to_base58(),
                    rtt.as_millis()
                );
            }
            ping::Event {
                peer,
                result: Result::Ok(ping::Success::Pong),
            } => {
                println!("ping: pong from {}", peer.to_base58());
            }
            ping::Event {
                peer,
                result: Result::Err(ping::Failure::Timeout),
            } => {
                println!("ping: timeout to {}", peer.to_base58());
            }
            ping::Event {
                peer,
                result: Result::Err(ping::Failure::Unsupported),
            } => {
                println!("ping: {} does not support ping protocol", peer.to_base58());
            }
            ping::Event {
                peer,
                result: Result::Err(ping::Failure::Other { error }),
            } => {
                println!("ping: ping::Failure with {}: {}", peer.to_base58(), error);
            }
        }
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct FloodsubMdnsBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for FloodsubMdnsBehaviour {
    fn inject_event(&mut self, message: FloodsubEvent) {
        if let FloodsubEvent::Message(msg) = message {
            println!("Received: '{:?}' from {:?}", String::from_utf8_lossy(&msg.data), msg.source);
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for FloodsubMdnsBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (pid, _) in list {
                    self.floodsub.add_node_to_partial_view(pid);
                }
            }
            MdnsEvent::Expired(list) => {
                for (pid, _) in list {
                    if !self.mdns.has_node(&pid) {
                        self.floodsub.remove_node_from_partial_view(&pid);
                    }
                }
            }
        }
    }
}
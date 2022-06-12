use libp2p::{
    NetworkBehaviour,
    gossipsub::{Gossipsub, GossipsubEvent},
    identify::{Identify, IdentifyEvent},
    ping,
    swarm::NetworkBehaviourEventProcess
};

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct Privnet {
    pub gossipsub: Gossipsub,
    pub identify: Identify,
    pub ping: ping::Behaviour,
}

// Define the network's identification behaviour
impl NetworkBehaviourEventProcess<IdentifyEvent> for Privnet {
    // Called when `identify` produces an event.
    fn inject_event(&mut self, event: IdentifyEvent) {
        println!("identify: {:?}", event);
    }
}

// Implement the network's Gossipsub Behaviour
impl NetworkBehaviourEventProcess<GossipsubEvent> for Privnet {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            } => println!(
                "Got message: {} with id: {} from peer: {:#?}",
                String::from_utf8_lossy(&message.data),
                id,
                peer_id
            ),
            _ => {}
        }
    }
}

// Define the network's ping behaviour
impl NetworkBehaviourEventProcess<ping::Event> for Privnet {
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
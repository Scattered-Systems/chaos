use libp2p::{
    floodsub::{Floodsub, FloodsubEvent},
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
    swarm::NetworkBehaviourEventProcess,
};

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
use std::{collections::HashMap, net::SocketAddr};

use libp2p::{
    core::{
        muxing::StreamMuxerBox,
        transport::Boxed,
        upgrade,
    },
    identity,
    mplex,
    noise,
    PeerId,
    tcp,
    Transport,
};

use crate::chain::peer::Peer;

#[derive(Clone, Debug)]
pub struct Node {
    pub addresses: HashMap<String, SocketAddr>,
}

impl Node {
    pub fn new() -> Self {
        let host = [0, 0, 0, 0];
        let ports = [8088, 8089, 8090];
        let labels = ["api", "client", "chain"];
        let mut addresses = HashMap::new();
        for i in 0..3 {
            addresses.insert(String::from(labels[i]), SocketAddr::from((host, ports[i])));
        }
        Self {
            addresses
        }
    }
}

#[derive(Clone, Debug)]
pub struct Provider {
    pub peer: Peer,
    pub transport: Boxed<(PeerId, StreamMuxerBox)>,
}

impl Provider {
    pub fn new(peer: &Peer) -> Self {
        let dh_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&peer.key.clone())
            .expect("Signing Error: Failed to sign the static DH KeyPair");

        let transport = tcp::TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(dh_keys.clone()).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        Self {
            peer: peer.clone(),
            transport: transport.clone(),
        }
    }
}
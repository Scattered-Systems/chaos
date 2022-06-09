/*
    Conduit
        Designed to be the standard method of establishing transport instances for the network;
        This structure is designed around a more robust asynchronous framework in Tokio.
*/

use acme::primitives::AuthenticatedStaticKeys;
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::Boxed, upgrade},
    mplex,
    noise,
    PeerId,
    tcp,
    Transport,
};

use crate::network::peer::Peer;

pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

#[derive(Clone)]
pub struct Conduit {
    dh_keys: AuthenticatedStaticKeys,
    pub transport: BoxedTransport,
}

impl Conduit {
    pub fn new(peer: &Peer) -> Self {
        // Generate a temporary, authenticated set of static keys for each instance of the Conduit
        let dh_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&peer.key.clone())
            .expect("Signing Error: Failed to sign the static DH KeyPair");

        // Implement a standard transport for the network leveraging the TokioTcpConfig setup
        let transport = tcp::TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(dh_keys.clone()).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();
        Self {
            dh_keys: dh_keys.clone(),
            transport: transport.clone(),
        }
    }
}
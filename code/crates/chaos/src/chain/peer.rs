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

#[derive(Clone, Debug)]
pub struct Provider {
    pub peer: Peer,
    pub transport: Boxed<(PeerId, StreamMuxerBox)>,
}

impl Provider {
    pub fn new(peer: Peer) -> Self {
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

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PeerId,
    pub key: identity::Keypair,
}

impl Peer {
    pub fn new() -> Self {
        let key = identity::Keypair::generate_ed25519();
        Self {
            id: PeerId::from(key.public().clone()),
            key: key.clone(),
        }
    }

    pub fn authorize(key: identity::Keypair) -> noise::AuthenticKeypair<noise::X25519Spec> {
        let nk = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&key.clone())
            .expect("Signing Error: Failed to sign the static DH KeyPair");
        return nk;
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nTimestamp: {}\nLocal Peer Id: {}\n", chrono::Local::now().to_string(), self.id)
    }
}
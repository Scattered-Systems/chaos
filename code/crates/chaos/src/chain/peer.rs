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

pub fn authorize(key: identity::Keypair) -> noise::AuthenticKeypair<noise::X25519Spec> {
    let dh_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&key.clone())
        .expect("Signing Error: Failed to sign the static DH KeyPair");
    return dh_keys
}

pub fn create_transport(dh_keys: noise::AuthenticKeypair<noise::X25519Spec>) -> Boxed<(PeerId, StreamMuxerBox)> {
    let transport = tcp::TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(dh_keys.clone()).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();
    return transport.clone()
}

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: PeerId,
    pub key: identity::Keypair
}

impl Peer {
    pub fn new() -> Self {
        let key = identity::Keypair::generate_ed25519();
        let id = PeerId::from(key.public().clone());

        Self {
            id: id.clone(),
            key: key.clone(),
        }
    }

    pub fn from(key: identity::Keypair) -> Self {
        Self {
            id: PeerId::from(key.public().clone()),
            key: key.clone(),
        }
    }

    pub fn transport(&self) -> Boxed<(PeerId, StreamMuxerBox)> {
        return create_transport(authorize(self.key.clone()).clone())
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nTimestamp: {}\nLocal Peer Id: {}\n", chrono::Local::now().to_string(), self.id)
    }
}
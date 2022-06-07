use libp2p::{
    identity,
    noise,
    PeerId,
};

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
        let nk = noise::Keypair::new()
            .into_authentic(key.clone())
            .expect("Signing Error: Failed to sign the static DH KeyPair");
        return nk;
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nTimestamp: {}\nLocal Peer Id: {}\n", chrono::Local::now().to_string(), self.id)
    }
}
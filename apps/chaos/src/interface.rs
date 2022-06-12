use acme::chain::networking::peer::Peer;
use acme::primitives::networking::BoxedTransport;

pub type KV<T> = std::collections::HashMap<String, T>;
pub type Length = usize;


pub struct Constants {
    pub prefixes: Vec<Pre>,
}

pub trait NetworkPeer {
    type Address;
    type ContentId;

    fn new() -> Self;
    fn build_transport(&mut self) -> acme::primitives::networking::BoxedTransport;
}

pub trait Node {
    type L;

    fn setup(&mut self) -> dyn Node;
    fn run(&mut self) -> Self;
}

pub struct Interface {
    pub peer: Peer,
}

impl NetworkPeer for Interface {
    type Address = ();
    type ContentId = ();

    fn new() -> Self {
        todo!()
    }

    fn build_transport(&mut self) -> BoxedTransport {
        todo!()
    }
}

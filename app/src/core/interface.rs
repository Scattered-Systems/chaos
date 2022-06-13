use disaronno::{
    networking::peers::Peer,
    types::BoxedTransport,
};

pub type Length = usize;


pub trait NetworkPeer {
    type Address;
    type ContentId;

    fn new() -> Self;
    fn build_transport(&mut self) -> BoxedTransport;
}

pub trait Node {
    type L;

    fn setup(&mut self) -> Self;
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

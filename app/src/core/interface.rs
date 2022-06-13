use disaronno::{
    networking::peers::Peer,
    types::BoxedTransport,
};

pub type Length = usize;


pub trait Node {
    type Behaviour;

    fn new() -> Self;
    fn setup(&mut self) -> Self;
    fn run(&mut self) -> Self;
}

pub struct Interface {
    pub peer: Peer,
}

impl Node for Interface {
    type Behaviour = ();

    fn new() -> Self {
        Self {
            peer: Peer::new()
        }
    }
    fn setup(&mut self) -> Self {
        todo!()
    }
    fn run(&mut self) -> Self {
        todo!()
    }
}

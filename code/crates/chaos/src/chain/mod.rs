pub mod blockchain;
pub mod node;
pub mod peer;

pub trait Connection {
    type Address;

    fn connect(&self) -> Self::Address;
}


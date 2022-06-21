use acme::Peer;
use clap::Parser;
use crate::{Configuration, Context};

#[derive(Clone, Debug)]
pub struct Interface {
    pub context: Context,
    pub peer: Peer
}

impl Interface {
    pub fn new(configuration: Configuration) -> Self {
        let context = Context::new(configuration.clone());
        let peer = Peer::new();
        Self {
            context: context.clone(),
            peer: peer.clone()
        }
    }
}

impl crate::CLI for Interface {
    type Commands = crate::Commands;

    fn constructor(&self) -> Self::Commands {
        return crate::Commands::parse()
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interface(context={})", self.context)
    }
}
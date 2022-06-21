use acme::Peer;
use clap::Parser;
use crate::{Configuration, Context};

#[derive(Clone, Debug)]
pub struct Interface {
    pub context: Context,
    pub peer: Peer
}

impl Interface {
    pub fn constructor(configuration: Configuration) -> Self {
        Self {
            context: Context::constructor(configuration.clone()),
            peer: Peer::new().clone()
        }
    }
}

impl crate::CLI for Interface {
    type Commands = crate::Commands;

    fn constructor() -> Self::Commands {
        return Self::Commands::parse()
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interface(context={})", self.context)
    }
}
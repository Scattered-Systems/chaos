/*
    Appellation: application
    Context:
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */
use crate::{Commands, Configuration};
use acme::Peer;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Context {
    pub configuration: Configuration,
}

impl Context {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration.clone()
        }
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context(configuration={})", self.configuration)
    }
}

#[derive(Clone, Debug)]
pub struct App {
    pub context: Context,
    pub peer: Peer,
}

impl App {
    pub fn new(configuration: Configuration) -> Self {
        let context = Context::new(configuration.clone());
        let peer = Peer::new();
        Self {
            context: context.clone(),
            peer: peer.clone(),
        }
    }
    pub fn client(&self) -> Commands {
        return Commands::parse().clone();
    }
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interface(context={})", self.context)
    }
}
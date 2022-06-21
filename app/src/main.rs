/*
    Author: Joe McCain III
    Email: jo3mccain@icloud.com
    Date: June 812, 2022
    Package: chaos
    Project: Chaos
    Version: 0.1.0

    Overview:
            Chaos is intended to be deployed at an Ethereum Name, scaffolding the framework for a
        highly customizable private network to be created for the user. This also serves a number of
        roles in the Scattered Ecosystem, enabling the gateway to leverage the full might of
        cloud-based technologies while furthering the experience by incorporating a number of
        userful IoT features.

    Quickstart
        Run the application in two terminals, if your computer allows for mDNS than the nodes
        will automatically connect enabling you to input the commands below to store values on your
        network
        Commands
            * GET <key> <value>
            * GET_PROVIDERS <key>
            * PUT <key>
            * PUT_PROVIDER <key>
 */
use disarray::{Configuration, Context, CLI};

use acme::Peer;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Commands {

    #[clap(default_value = "chaos", long, short, value_parser)]
    pub appellation: String,

    #[clap(default_value = "scaffold", long, value_parser)]
    pub chain: String,

    #[clap(default_value = "false", long, value_parser)]
    pub cluster: String,


    #[clap(default_value = "", long, short, value_parser)]
    pub data: String

}

#[derive(Clone, Debug)]
pub struct App {
    pub context: Context,
    pub peer: Peer
}

impl App {
    pub fn new(configuration: Configuration) -> Self {
        let context = Context::new(configuration.clone());
        let peer = Peer::new();
        Self {
            context: context.clone(),
            peer: peer.clone()
        }
    }
}

impl CLI for App {
    type Commands = Commands;

    fn constructor(&self) -> Self::Commands {
        return Commands::parse()
    }
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interface(context={})", self.context)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // TODO - Create a standard, asynchronous configurator for network nodes
    let settings = match Configuration::new() {
        Ok(value) => value,
        Err(err) => panic!("ConfigurationError: {:#?}", err)
    };
    let interface = App::new(settings.clone());
    let args = interface.constructor();
    println!("{:#?}", args);
    Ok(())
}
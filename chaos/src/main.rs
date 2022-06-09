/*
    Author: Joe McCain III
    Email: jo3mccain@icloud.com
    Date: June 8, 2022
    Package: chaos
    Project: Chaos
    Version: 0.1.0

    Overview:
            Chaos is intended to be deployed at an Ethereum Name, scaffolding the framework for a
        highly customizable private network to be created for the user. This also serves a number of
        roles in the Scattered Ecosystem, enabling the gateway to leverage the full might of
        cloud-based technologies while furthering the experience by incorporating a number of
        userful IoT features.
 */

use acme::primitives::StandardError;

use crate::{controller::settings::Settings, network::node::Node};

mod apps;
mod consensus;
mod controller;
mod data;
mod network;

#[tokio::main]
async fn main() -> Result<(), StandardError> {
    // TODO - Create a standard, asynchronous configurator and integrated into the interface
    let settings = match Settings::new() {
        Ok(value) => value,
        Err(err) => panic!("ConfigurationError: {:#?}", err)
    };
    println!("{:#?}", settings.clone());

    // Create an node instance to run a self-hosted decentralized, private Virtual File System
    let instance = Node::new();
    Node::run(&instance).await?;
    Ok(())
}
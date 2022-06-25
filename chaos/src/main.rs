/*
    Appellation: Chaos
    Context: binary
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Chaos is designed to implement the storage capabilities of the SCSYS Ecosystem.
        Built leveraging libp2p and other custom support crates, Chaos results in a simple to use
        blockchain optimized for the end-user experience without making traditional sacrifices in
        terms of security.
 */
pub use crate::{
    application::*,
    commands::*,
    configuration::Configuration,
};

mod application;
mod commands;
mod configuration;

fn main() {
    let settings = match Configuration::new() {
        Ok(value) => value,
        Err(err) => panic!("ConfigurationError: {:#?}", err)
    };
    println!("{:#?}", settings);
    let interface = App::new(settings.clone());
    let args = interface.client();
    println!("{:#?}", args);
}
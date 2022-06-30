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
use chaos_sdk::CommandLineInterface;
pub use crate::{
    core::*,
    interface::*,
};

mod api;
mod core;
mod interface;

fn main() {
    let settings = Settings::new().ok().unwrap();
    let interface = App::new(settings.application.name.clone());

    println!("{:#?}", settings.clone());
    println!("{:#?}", &interface.client());
}
use std::error::Error;

use crate::interface::Interface;
use crate::settings::Settings;

mod blockchain;
mod data;
mod interface;
mod network;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let settings = match Settings::new() {
        Ok(value) => value,
        Err(err) => panic!("ConfigurationError: {:#?}", err)
    };
    println!("{:#?}", settings.clone());

    let interface = Interface::new();
    Interface::run(&interface).await?;
    Ok(())
}
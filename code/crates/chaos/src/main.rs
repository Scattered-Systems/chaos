use std::error::Error;

use crate::interface::Interface;
use crate::settings::Settings;

mod chain;
mod data;
mod interface;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let settings = match Settings::new() {
        Ok(value) => value,
        Err(err) => panic!("ConfigurationError: {:#?}", err)
    };
    let interface = Interface::new(settings);
    Interface::run(&interface).await?;
    Ok(())
}
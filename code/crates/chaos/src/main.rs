use std::error::Error;

use crate::chains::interface::Interface;
use crate::settings::Settings;

mod chains;
mod data;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let settings = match Settings::new() {
        Ok(value) => value,
        Err(err) => panic!("ConfigurationError: {:#?}", err)
    };
    prinln!("{:#?}", settings.clone());

    let interface = Interface::new();
    Interface::run(&interface).await?;
    Ok(())
}
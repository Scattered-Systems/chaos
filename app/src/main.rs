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
use disarray::{Interface, Settings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // TODO - Create a standard, asynchronous configurator for network nodes
    let settings = match Settings::new() {
        Ok(value) => value,
        Err(err) => panic!("ConfigurationError: {:#?}", err)
    };
    let interface = Interface::new(settings.clone());
    println!("{:#?}", &interface);
    Ok(())
}
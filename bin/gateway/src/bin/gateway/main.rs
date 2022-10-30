/*
    Appellation: scsys-gateway <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{context::Context, interface::Application, settings::Settings};
pub mod api;
pub(crate) mod context;
pub(crate) mod interface;
pub(crate) mod settings;

use scsys::BoxResult;
use scsys_gateway::gateways::{Gateway, simple_creds, simple_region};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> BoxResult {
    println!("Hello, world!");
    let access_key = std::env::var("STORJ_ACCESS_KEY")?;
    let secret_key = std::env::var("STORJ_SECRET_KEY")?;
    let endpoint = "https://gateway.storjshare.io";
    let region = "us-east-1";
    let creds = simple_creds(access_key.as_str(), secret_key.as_str());
    
    let gateway = Gateway::new(creds, simple_region(endpoint, region));
    let bucket = gateway.bucket("scsys")?;
    let objects = bucket.list("/lib/documents/research/".to_string(), Some("/".to_string())).await?;
    let object_names = objects.iter().map(|i| i.clone().name ).collect::<Vec<String>>();
    println!("{:?}", objects);

    let app = Application::default();
    app.with_logging();
    app.run().await?;

    Ok(())
}




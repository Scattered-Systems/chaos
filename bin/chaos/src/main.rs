/*
    Appellation: chaos <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

use s3::{creds::Credentials, Region};

pub use self::core::*;

pub mod app;
mod core;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    println!("Welcome to the Chaos!");

    let chaos = app::Chaos::default();
  

    Ok(())
}



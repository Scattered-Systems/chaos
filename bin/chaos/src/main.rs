/*
    Appellation: chaos <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

pub mod app;
pub mod core;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    println!("Welcome to the Chaos!");

    let chaos = app::Chaos::default();
    chaos.with_logging();
    chaos.spawn_rpc().await.expect("RPC Error");
    chaos.run().await.expect("App failed to start");

    Ok(())
}

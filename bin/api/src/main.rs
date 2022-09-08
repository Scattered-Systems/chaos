/*
   Appellation: app <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Maximus is a complete restful api template written in Rust, powered by Axum and Tokio.

*/
pub use self::{app::*, core::*, data::*};

mod app;
mod core;
mod data;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    let app = Backend::new();
    app.with_logging()
        .run()
        .await
        .expect("Application startup failed...");
    Ok(())
}

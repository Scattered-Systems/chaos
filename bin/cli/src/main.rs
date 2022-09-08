/*
   Appellation: app <binary>
   Creator: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
   Description:
        ... Summary ...

*/
pub mod app;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    println!("Hello, world!");
    let app = app::Application::new();

    app.run().await.expect("Application Error");
    
    Ok(())
}


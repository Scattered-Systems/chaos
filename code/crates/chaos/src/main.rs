mod apps;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let rpc = apps::api::interface::Interface::new().await;
    Ok(())
}